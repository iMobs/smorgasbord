use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    async fn get_greeting(&self) -> String {
        "Hello World!".to_owned()
    }
}
