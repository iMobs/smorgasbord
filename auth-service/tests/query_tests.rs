use common::{test_graphql_request, GraphqlRequest};

mod common;

#[actix_rt::test]
async fn test_greeting() {
    let query = r#"
		query {
			getGreeting
		}
	"#
    .to_string();

    let request_body = GraphqlRequest {
        query,
        variables: None,
    };

    let response = test_graphql_request(request_body).await;

    assert_eq!(
        response
            .data
            .expect("Response has no data")
            .get("getGreeting")
            .expect("Can't get property")
            .as_str()
            .expect("Can't get property as str"),
        "Hello World!"
    );
}
