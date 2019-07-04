use crate::db::{Pool, establish_connection};
use actix_web::{App, HttpServer, Result};
use actix_web::middleware;
use actix_web::web;
use actix_cors::Cors;
use crate::api::user;
use actix_redis::RedisActor;

pub fn data(cfg: &mut web::ServiceConfig) {
    let db = establish_connection().unwrap();
    let redis = RedisActor::start("127.0.0.1:6379");
    cfg.data(db)
        .data(redis);
}

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/user/signup").route(web::post().to_async(user::user_signup)))
            .service(web::resource("/user/valid_captcha").route(web::post().to_async(user::valid_captcha)))
            .service(web::resource("/user/check_phone").route(web::post().to_async(user::check_phone)))
            .service(web::resource("/user/login").route(web::post().to_async(user::user_login)))
            .service(web::resource("/user/user_info").route(web::post().to_async(user::user_info)))
    );
}