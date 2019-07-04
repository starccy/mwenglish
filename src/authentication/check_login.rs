use actix_web::{FromRequest, HttpRequest, Error, HttpResponse};
use actix_web::web::{Bytes, JsonConfig};
use actix::dev::Stream;
use actix_web::dev::Payload;
use futures::Future;
use futures::future::{ok as fut_ok, err as fut_err, Either};
use actix_http::httpmessage::HttpMessage;
use crate::share::error::MyError;
use actix_web::error::{ResponseError, PayloadError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoggedUser {
    pub token: String,
    pub s_key: String,
}

impl FromRequest for LoggedUser {
    type Error = HttpResponse;
//    type Future = Either<Box<dyn Future<Item = Self, Error = Self::Error>>, Box<dyn Future<Item = Self, Error = Self::Error>>>;
    type Future = Box<dyn Future<Item = Self, Error = Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload<Box<dyn Stream<Item=Bytes, Error=PayloadError>>>) -> Self::Future {
//        let mime = req.mime_type();
//        let is_json = match mime {
//            Ok(mime_type) => {
//                match mime_type {
//                    Some(x) => {
//                        if x.subtype() == "json" {
//                            true
//                        }
//                        else {
//                            false
//                        }
//                    },
//                    _ => false
//                }
//            },
//            _ => false
//        };
        Box::new(fut_err(MyError::Unauthorized.error_response()))
    }
}