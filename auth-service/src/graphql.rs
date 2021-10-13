use async_graphql::*;
use async_graphql::{Context, EmptySubscription, Object, Schema};

use crate::get_conn_from_ctx;
use crate::models::{NewUser as NewUserEntity, User as UserEntity};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    async fn get_greeting(&self) -> String {
        "Hello World!".to_owned()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_user(&self, ctx: &Context<'_>, user: UserInput) -> Result<User> {
        let new_user = NewUserEntity {
            email: user.email,
            password: user.password,
        };

        let created_user_entity =
            crate::repository::create_user(new_user, &get_conn_from_ctx(&ctx))?;

        Ok(User::from(&created_user_entity))
    }
}

#[derive(SimpleObject)]
struct User {
    email: String,
}

#[derive(InputObject)]
struct UserInput {
    email: String,
    password: String,
}

impl From<&UserEntity> for User {
    fn from(entity: &UserEntity) -> Self {
        User {
            email: entity.email.clone(),
        }
    }
}
