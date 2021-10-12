use actix_web::{web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_actix_web::{Request, Response};

type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

struct Query;

#[Object]
impl Query {
    async fn get_greeting(&self) -> String {
        "Hello World!".to_owned()
    }
}

fn configure_service(cfg: &mut web::ServiceConfig) {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema: AppSchema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    HttpServer::new(move || App::new().configure(configure_service).data(schema.clone()))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
