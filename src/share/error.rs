use actix_web::error::ResponseError;
use actix_web::HttpResponse;
use actix_web::Error;
use crate::share::common::BaseDataMsgs;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum MyError {
    Unauthorized
}


impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::Unauthorized => {
                HttpResponse::Ok().json(BaseDataMsgs::<String>::not_login())
            }
        }
    }
}

