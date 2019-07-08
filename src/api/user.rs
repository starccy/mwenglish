use crate::db::Pool;
use actix_web::{web, HttpRequest};
use actix_web::{HttpResponse, Error};
use futures::future::{Future, ok as fut_ok, Either};
use crate::model::user::*;
use crate::authentication::vaptcha::{ValidatePayload, ValidateToken};
use crate::share::common::{BaseDataMsgs, StringDataMsgs};
use actix::Addr;
use actix_redis::{RedisActor, Command};
use redis_async::resp::RespValue;
use crate::utils::{gen_sms_code, send_sms};
use crate::crypt::create_uuid;
use crate::crypt::rsa::ReaKeyPair;
use crate::authentication::check_login::{LoggedUser, RequestData, UserUid};

pub fn user_signup(db: web::Data<Pool>, signup_user: web::Json<SignUpUser>, redis: web::Data<Addr<RedisActor>>) -> impl Future<Item = HttpResponse, Error = Error> {
    if *&signup_user.password.is_empty() || *&signup_user.password.len() < 4 || *&signup_user.password.len() > 18 {
        return Either::A(fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<String>::fail_with_message("密码格式不正确"))));
    }
    Either::B(redis.send(Command(resp_array!["GET", &signup_user.phone]))
        .from_err()
        .and_then(move |res| {
            match &res {
                Ok(RespValue::BulkString(x)) => {
                    let x = String::from_utf8_lossy(x);
                    if &*x == &signup_user.code {
                        if signup_user.register(&*db) {
                            fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<Option<String>>::success_with_message(None, "新用户注册成功")))
                        }
                        else {
                            fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<Option<String>>::fail_with_message("新用户注册失败")))
                        }
                    }
                    else {
                        fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<String>::fail_with_message("验证码失效或有误")))
                    }
                },
                Ok(RespValue::Nil) => {
                    fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<String>::fail_with_message("验证码失效或有误")))
                }
                _ => {
                    fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<String>::exception()))
                }
            }
        }))
}

pub fn check_phone(db: web::Data<Pool>, phone: web::Json<UserPhone>) -> impl Future<Item = HttpResponse, Error = Error> {
    if ! phone.is_valid() {
        return fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<String>::fail_with_message("手机号码格式不正确")));
    }
    if phone.is_exist(&db.clone()) {
        return fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<String>::fail_with_message("该手机号码已被注册")));
    }
    fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<Option<String>>::success(None)))
}

pub fn valid_captcha(db: web::Data<Pool>, redis: web::Data<Addr<RedisActor>>, valid_token: web::Json<ValidateToken>) -> impl Future<Item = HttpResponse, Error = Error> {
    let token = &valid_token.token;
    let phone = UserPhone {phone: valid_token.phone.to_owned()};

    if ! phone.is_valid() {
        return Either::A(fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<String>::fail_with_message("手机号码格式不正确"))));
    }
    if phone.is_exist(&db.clone()) {
        return Either::A(fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<String>::fail_with_message("该手机号码已被注册"))));
    }

    if token.is_empty() {
        return Either::A(fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<String>::fail_with_message("请进行手势验证"))));
    }
    let payload = ValidatePayload::new(token.to_owned());
    let success = payload.is_valid();
    if ! success {
        return Either::A(fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<String>::fail_with_message("手势验证码验证失败"))));
    }
    let sms_code = gen_sms_code();
    Either::B(redis.send(Command(resp_array!["SET", &*phone, &sms_code, "EX", "300"]))
        .from_err()
        .and_then(move |res| {
            match &res {
                Ok(_) => {
                    let send_status = send_sms(phone.phone, sms_code.clone());
                    if send_status.is_err() {
                        return fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<Option<String>>::exception_with_message(&send_status.err().unwrap())));
                    }
                    else {
                        return fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<Option<String>>::success(None)));
                    }
                },
                _ => return fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<Option<String>>::exception()))
            }
        }))
}

pub fn user_login(db: web::Data<Pool>, redis: web::Data<Addr<RedisActor>>, login_user: web::Json<LoginUser>) -> impl Future<Item = HttpResponse, Error = Error> {
    if *&login_user.password.is_empty() || *&login_user.phone.is_empty() {
        return Either::A(fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<String>::fail_with_message("用户名或密码不能为空"))));
    }
    let uid = login_user.get_uid(&*db);
    if uid.is_empty() {
        return Either::A(fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<String>::fail_with_message("用户名或密码错误"))));
    }
    let token = create_uuid();
    let key_pair = ReaKeyPair::new();
    let pk_key = key_pair.public_key_string();
    let pv_key = key_pair.private_key_string();
    let redis_user_session = RedisUserSession {
        uid: uid.clone(),
        pk_key: pk_key.clone(),
        pv_key: pv_key.clone(),
    };
    let redis_user_session_str = serde_json::to_string(&redis_user_session).unwrap();
    Either::B(redis.send(Command(resp_array!["SET", &token, &redis_user_session_str, "EX", "7200"]))
        .from_err()
        .and_then(move |res| {
            match &res {
                Ok(_) => {
                    let user_session = UserSession {
                        token,
                        pk_key
                    };
                    fut_ok(HttpResponse::Ok().json(BaseDataMsgs::success(&user_session)))
                },
                _ => fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<String>::exception()))
            }
        }))
}

pub fn user_info(db: web::Data<Pool>, redis: web::Data<Addr<RedisActor>>, logged_user: RequestData<LoggedUser>, req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    let extensions = req.extensions();
    let uid = extensions.get::<UserUid>().clone();
    if uid.is_none() {
        dbg!("user id is none");
    }
    else {
        dbg!(uid);
    }
    fut_ok(HttpResponse::Ok().json(BaseDataMsgs::<Option<String>>::success(None)))
}


