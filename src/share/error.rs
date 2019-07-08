use actix_web::error::ResponseError;
use actix_web::HttpResponse;
use crate::share::common::{StringDataMsgs, BaseDataMsgs};
use derive_more::Display;
use serde::export::fmt::{Debug, Display};
use serde::Serialize;
use actix_http::Response;
use actix_web::dev::Body;
use serde_json::to_string_pretty;
use std::fmt::{Formatter, Error};

#[derive(Debug, Display)]
pub enum MyError<T: Debug + Display> {
    Unauthorized,
    #[display(fmt = "ErrMsg")]
    ErrMsg(StringDataMsgs<T>)
}


impl<T: Debug + Display + Serialize> ResponseError for MyError<T> {
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::Unauthorized => {
                HttpResponse::Ok().json(StringDataMsgs::from(BaseDataMsgs::<String>::not_login()))
            },
            MyError::ErrMsg(msg) => {
                HttpResponse::Ok().json(msg)
            }
        }
    }
}

impl<'a> Display for StringDataMsgs<Option<String>> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl<'a> ResponseError for StringDataMsgs<Option<String>> {
    fn render_response(&self) -> Response<Body> {
        HttpResponse::Ok().json(&self)
    }
}