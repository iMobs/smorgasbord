use actix_web::{App, HttpServer};
use dotenv::dotenv;

use auth_service::{configure_service, create_connection_pool, create_schema, run_migrations};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = create_connection_pool();

    run_migrations(&pool);

    let schema = create_schema(pool);

    HttpServer::new(move || App::new().configure(configure_service).data(schema.clone()))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
