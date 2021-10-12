use actix_web::{test, App};
use serde::{Deserialize, Serialize};
use serde_json::Map;

use auth_service::{configure_service, create_schema};

#[actix_rt::test]
async fn test_greeting() {
    let mut service = test::init_service(
        App::new()
            .configure(configure_service)
            .data(create_schema()),
    )
    .await;

    let query = r#"
		query {
			getGreeting
		}
	"#
    .to_string();

    let request_body = GraphqlCustomRequest { query, variables: None };

    let request = test::TestRequest::post()
        .uri("/")
        .set_json(&request_body)
        .to_request();

    let response: GraphqlCustomResponse = test::read_response_json(&mut service, request).await;

    assert_eq!(
        response.data.get("getGreeting")
            .expect("Can't get property")
            .as_str()
            .expect("Can't get property as str"),
        "Hello World!"
    );
}

#[derive(Serialize)]
struct GraphqlCustomRequest {
    query: String,
		variables: Option<Map<String, serde_json::Value>>,
}

#[derive(Deserialize)]
struct GraphqlCustomResponse {
    data: serde_json::Value,
}
