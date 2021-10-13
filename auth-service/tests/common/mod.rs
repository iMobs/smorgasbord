use actix_web::{test, App};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use auth_service::{configure_service, create_connection_pool, create_schema, run_migrations};

pub async fn test_graphql_request(request_body: GraphqlRequest) -> GraphqlResponse {
    dotenv().ok();
    let pool = create_connection_pool();
    run_migrations(&pool);
    let mut service = test::init_service(
        App::new()
            .configure(configure_service)
            .data(create_schema(pool)),
    )
    .await;

    let request = test::TestRequest::post()
        .uri("/")
        .set_json(&request_body)
        .to_request();

    test::read_response_json(&mut service, request).await
}

#[derive(Serialize)]
pub struct GraphqlRequest {
    pub query: String,
    pub variables: Option<Map<String, Value>>,
}

#[derive(Deserialize)]
pub struct GraphqlResponse {
    pub data: Option<Value>,
    pub error: Option<Value>,
}
