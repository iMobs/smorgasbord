use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;
use env_logger::{Builder, Env};

use auth_service::{configure_service, create_connection_pool, create_schema, run_migrations};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let pool = create_connection_pool();

    run_migrations(&pool);

    let schema = create_schema(pool);

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .configure(configure_service)
            .app_data(Data::new(schema.clone()))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
