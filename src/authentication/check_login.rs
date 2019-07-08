use actix_web::{FromRequest, HttpRequest};
use actix::dev::Stream;
use actix_web::dev::Payload;
use futures::Future;
use actix::Addr;
use actix_redis::{RedisActor, Command, RespValue};
use crate::share::common::{BaseDataMsgs, StringDataMsgs};
use json::JsonValue;
use crate::model::user::RedisUserSession;
use std::ops::Deref;
use serde::de::DeserializeOwned;
use crate::crypt::aes::AesKey;
use crate::crypt::rsa::ReaKeyPair;
use serde::export::fmt::Debug;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoggedUser {
    pub token: String,
    pub s_key: String,
    pub data: String,
}

#[derive(Clone, Debug)]
pub struct UserUid(pub String);

impl Deref for UserUid {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct RequestData<T>(pub T);

impl<T> Deref for RequestData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> FromRequest for RequestData<T>
//impl FromRequest for RequestData<LoggedUser>
    where
        T: DeserializeOwned + Debug + 'static,
{
    type Error = StringDataMsgs<Option<String>>;
    type Future = Box<dyn Future<Item = Self, Error = Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let redis = req.app_data::<Addr<RedisActor>>().unwrap().to_owned();
        let payload = payload.take();
        let req2 = req.to_owned();
        Box::new(payload.concat2()
            .map_err(|_| StringDataMsgs::<Option<String>>::from(BaseDataMsgs::exception()))
            // extract request body to string
            .and_then(|body| {
                let result = json::parse(std::str::from_utf8(&body).unwrap());
                let in_json: JsonValue = match result {
                    Ok(v) => v,
                    Err(e) => json::object! {"error" => e.to_string()},
                };
                Ok(in_json.to_string())
            })
            // convert the string to LoggedUser struct
            .and_then(move |json_str| {
                let logged_user = serde_json::from_str::<LoggedUser>(&json_str);
                if logged_user.is_err() {
                    Err(StringDataMsgs::from(BaseDataMsgs::format_error()))
                }
                else {
                    Ok(logged_user)
                }
            })
            // verify current user,
            // then decrypted the data
            // convert data to specific struct
            .and_then( move |logged_user| {
                let logged_user = logged_user.unwrap();
                let token = logged_user.token.clone();
                // get user information from redis with token
                redis.send(Command(resp_array!["GET", &token]))
                    .map_err(|_| StringDataMsgs::from(BaseDataMsgs::exception_with_message("redis server error")))
                    .and_then(move |response| {
                        match &response {
                            // if user is valid in current session
                            Ok(RespValue::BulkString(x)) => {
                                let x = String::from_utf8_lossy(x);
                                let redis_user_session = serde_json::from_str::<RedisUserSession>(&*x).unwrap();
                                // set user id to HttpRequest
                                let mut extensions = req2.extensions_mut();
                                extensions.insert(UserUid(redis_user_session.uid.clone()));
                                let rsa_key = ReaKeyPair::new_with_params(Some(redis_user_session.pv_key), None);
                                dbg!(&rsa_key);
                                if rsa_key.is_err() {
                                    return Err(StringDataMsgs::from(BaseDataMsgs::format_error_with_message(&rsa_key.err().unwrap())));
                                }
                                let rsa_key = rsa_key.unwrap();
                                let aes_key_str = rsa_key.decrypt_with_private_key(logged_user.s_key.clone());
                                dbg!(&aes_key_str);
                                if aes_key_str.is_err() {
                                    return Err(StringDataMsgs::from(BaseDataMsgs::format_error_with_message(&aes_key_str.err().unwrap())));
                                }
                                let aes_key_str = aes_key_str.unwrap();
                                let aes_key = AesKey::set_key_from_base64(aes_key_str);
                                if aes_key.is_err() {
                                    return Err(StringDataMsgs::from(BaseDataMsgs::format_error_with_message(&aes_key.err().unwrap())));
                                }
                                let aes_key = aes_key.unwrap();
                                // decode data from request
                                let data = aes_key.decrypt(logged_user.data.clone());
                                dbg!(&data);
                                if data.is_err() {
                                    return Err(StringDataMsgs::from(BaseDataMsgs::format_error_with_message(&data.err().unwrap())));
                                }
                                let data = data.unwrap();
                                dbg!(&data);
                                // convert data to specific struct
                                let result = serde_json::from_str::<T>(&data).map_err(|e| e.to_string());
                                dbg!(&result);
                                if result.is_err() {
                                    return Err(StringDataMsgs::from(BaseDataMsgs::format_error_with_message(&result.err().unwrap())));
                                }
                                let result = result.unwrap();
                                Ok(result)
                            },
                            _ => {
                                return Err(StringDataMsgs::from(BaseDataMsgs::not_login()));
                            }
                        }
                    })
            })
            // make T into RequestData
            .map(RequestData)
        )
    }
}