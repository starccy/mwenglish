use chrono::{NaiveDateTime, Local};
use crate::share::schema::users;
use diesel::prelude::*;
use crate::db::Pool;
use std::ops::Deref;
use crate::crypt;
use actix::Addr;
use actix_redis::{RedisActor, Command};
use futures::future::Future;
use redis_async::resp::RespValue;

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[table_name = "users"]
#[primary_key(id)]
pub struct User {
    #[column_name = "id"] pub id: String,
    #[column_name = "phone"] pub phone: String,
    #[column_name = "password"] pub password: String,
    #[column_name = "nickname"] pub nickname: Option<String>,
    #[column_name = "avatar"] pub avatar: Option<String>,
    #[column_name = "is_admin"] pub is_admin: i8,
//    #[column_name = "is_active"] pub is_active: i8,
//    #[column_name = "sms_code"] pub sms_code: Option<String>,
    #[column_name = "created_at"] pub created_at: NaiveDateTime,
    #[column_name = "last_modified_at"] pub last_modified_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignUpUser {
    pub phone: String,
    pub password: String,
    pub code: String,
}

impl SignUpUser {
    pub fn register(&self, db: &Pool) -> bool {
        let conn = db.get().unwrap();
        let user = User {
            id: crypt::create_uuid(),
            phone: self.phone.clone(),
            password: crypt::create_md5(self.password.clone()).unwrap(),
            nickname: Some(self.phone.clone()),
            avatar: None,
            is_admin: 0,
            created_at: Local::now().naive_local(),
            last_modified_at: Local::now().naive_local()
        };
        let result = diesel::insert_into(users::table).values(&user).execute(&conn).unwrap_or(0);
        result != 0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPhone {
    pub phone: String,
}

impl UserPhone {
    pub fn is_valid(&self) -> bool {
        let reg = regex::Regex::new(r#"^1(3|4|5|6|7|8|9)\d{9}$"#).unwrap();
        reg.is_match(&self.phone)
    }

    pub fn is_exist(&self, db: &Pool) -> bool {
        let conn = db.get().unwrap();
        let user = users::table.filter(users::phone.eq(&self.phone)).first::<User>(&conn);
        user.is_ok()
    }
}

impl Deref for UserPhone {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.phone
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    pub phone: String,
    pub password: String,
}

impl LoginUser {
    pub fn get_uid(&self, db: &Pool) -> String {
        let conn = db.get().unwrap();
        let user = users::table.filter(users::phone.eq(&self.phone))
            .filter(users::password.eq(crypt::create_md5((&self.password).to_owned()).unwrap()))
            .first::<User>(&conn);
        if user.is_err() {
            "".to_string()
        }
        else {
            user.unwrap().id
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisUserSession {
    pub uid: String,
    pub pk_key: String,
    pub pv_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub token: String,
    pub pk_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub phone: String,
    pub nickname: String,
    pub avatar: String,
}

impl UserInfo {
    pub fn get_by_token(db: &Pool, redis: &Addr<RedisActor>, token: &String) -> impl Future<Item = Option<UserInfo>, Error = actix::MailboxError> {
        let conn = db.get().unwrap();
        redis.send(Command(resp_array!["GET", token]))
            .and_then(move |res| {
                match &res {
                    Ok(RespValue::BulkString(x)) => {
                        let session_str = &*String::from_utf8_lossy(x);
                        let redis_session = serde_json::from_str::<RedisUserSession>(session_str).unwrap();
                        let user = users::table.filter(users::id.eq(redis_session.uid)).first::<User>(&conn);
                        let user = user.unwrap();
                        futures::future::ok(Some(UserInfo {
                            phone: user.phone,
                            nickname: user.nickname.unwrap_or_default(),
                            avatar: user.avatar.unwrap_or_default()
                        }))
                    },
                    _ => futures::future::ok(None)
                }
            })
    }
}