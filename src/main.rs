#[macro_use]
extern crate actix_web;

use std::{env, io};
use actix_web::{middleware, web, App, HttpServer, HttpResponse};

async fn get_health_status() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("Service en ligne ✅")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(get_health_status))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
