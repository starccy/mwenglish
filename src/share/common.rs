use crate::db::Pool;
use actix::{Addr, Actor, SyncContext};
use actix_redis::RedisActor;

//pub struct AppState {
//    pub db: Pool,
//    pub redis: Addr<RedisActor>,
//}
//
//impl Actor for AppState {
//    type Context = SyncContext<Self>;
//}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseDataMsgs<'a, T> {
    pub code: &'a str,
    pub msg: &'a str,
    pub data: T,
}

impl<'a, T> BaseDataMsgs<'a, T> {
    pub fn success(data: T) -> BaseDataMsgs<'a, T> {
        BaseDataMsgs {
            code: "0",
            msg: "请求成功",
            data
        }
    }

    pub fn success_with_message(data: T, msg: &'a str) -> BaseDataMsgs<T> {
        BaseDataMsgs {
            code: "0",
            msg,
            data
        }
    }

    pub fn fail() -> BaseDataMsgs<'a, Option<T>> {
        BaseDataMsgs {
            code: "1",
            msg: "操作失败",
            data: None
        }
    }

    pub fn fail_with_message(msg: &'a str) -> BaseDataMsgs<'a, Option<T>> {
        BaseDataMsgs {
            code: "1",
            msg,
            data: None
        }
    }

    pub fn not_login() -> BaseDataMsgs<'a, Option<T>> {
        BaseDataMsgs {
            code: "2",
            msg: "未登录",
            data: None
        }
    }

    pub fn not_login_with_message(msg: &'a str) -> BaseDataMsgs<'a, Option<T>> {
        BaseDataMsgs {
            code: "2",
            msg,
            data: None
        }
    }

    pub fn exception() -> BaseDataMsgs<'a, Option<T>> {
        BaseDataMsgs {
            code: "3",
            msg: "业务异常",
            data: None
        }
    }

    pub fn exception_with_message(msg: &'a str) -> BaseDataMsgs<'a, Option<T>> {
        BaseDataMsgs {
            code: "3",
            msg,
            data: None
        }
    }

    pub fn no_data() -> BaseDataMsgs<'a, Option<T>> {
        BaseDataMsgs {
            code: "4",
            msg: "未查询到数据",
            data: None
        }
    }

    pub fn no_data_with_message(msg: &'a str) -> BaseDataMsgs<'a, Option<T>> {
        BaseDataMsgs {
            code: "4",
            msg,
            data: None
        }
    }

    pub fn format_error() -> BaseDataMsgs<'a, Option<T>> {
        BaseDataMsgs {
            code: "5",
            msg: "请求数据格式不正确",
            data: None,
        }
    }

    pub fn format_error_with_message(msg: &'a str) -> BaseDataMsgs<'a, Option<T>> {
        BaseDataMsgs {
            code: "5",
            msg,
            data: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringDataMsgs<T> {
    pub code: String,
    pub msg: String,
    data: T,
}

impl<'a, T> From<BaseDataMsgs<'a, T>> for StringDataMsgs<T> {
    fn from(base: BaseDataMsgs<'a, T>) -> Self {
        Self {
            code: base.code.to_string(),
            msg: base.msg.to_string(),
            data: base.data
        }
    }
}

lazy_static! {
    pub static ref REDIS_PASS: String = {
        dotenv::dotenv().ok();
        let password: String = std::env::var("REDIS_PASS").expect("REDIS_PASS must be set");
        password
    };
}