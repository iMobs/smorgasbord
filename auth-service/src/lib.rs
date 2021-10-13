#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{web, HttpResponse, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;

use crate::graphql::{AppSchema, Query};

embed_migrations!();

pub mod graphql;
pub mod models;
pub mod schema;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_connection_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub fn run_migrations(pool: &PgPool) {
    let conn = pool.get().expect("Can't get DB connection");
    embedded_migrations::run(&conn).expect("Failed to run database migrations")
}

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(index))
            .route(web::get().to(index_playground)),
    );
}

async fn index(schema: web::Data<AppSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

pub fn create_schema(pool: PgPool) -> AppSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .enable_federation()
        .data(pool)
        .finish()
}
