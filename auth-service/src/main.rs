extern crate auth_service;

use actix_web::{App, HttpServer};
use auth_service::{configure_service, create_schema};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = create_schema();

    HttpServer::new(move || App::new().configure(configure_service).data(schema.clone()))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
