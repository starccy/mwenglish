#[macro_use] extern crate actix_web;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate redis_async;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate derive_more;

mod db;
mod share;
mod router;
mod crypt;
mod model;
mod utils;
mod authentication;
mod api;
mod handler;

use actix_web::{Result, Error, HttpServer, App};

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_redis=info");
    env_logger::init();
    let sys = actix_rt::System::new("mwenglish");

    HttpServer::new(move || {
        App::new()
            .configure(router::data)
            .wrap(actix_cors::Cors::default())
            .wrap(actix_web::middleware::Logger::default())
            .configure(router::router)
    })
        .bind("0.0.0.0:8000")?
        .run()
        .unwrap();

    sys.run().map_err(Error::from)
}
