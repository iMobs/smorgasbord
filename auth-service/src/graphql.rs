use async_graphql::*;
use async_graphql::{Context, EmptySubscription, Object, Schema};

use crate::get_conn_from_ctx;
use crate::models::{NewUser as NewUserEntity, User as UserEntity};
use crate::utils::{hash_password, verify_password};

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
    async fn signup(&self, ctx: &Context<'_>, user: UserInput) -> Result<User> {
        let new_user = NewUserEntity {
            email: user.email,
            password: hash_password(&user.password)?,
        };

        let created_user_entity =
            crate::repository::create_user(new_user, &get_conn_from_ctx(&ctx))?;

        Ok(User::from(&created_user_entity))
    }

    async fn login(&self, ctx: &Context<'_>, email: String, password: String) -> Result<bool> {
        let maybe_user =
            crate::repository::get_user_by_email(&email, &get_conn_from_ctx(&ctx)).ok();

        if let Some(user) = maybe_user {
            if let Ok(matching) = verify_password(&user.password, &password) {
                if matching {
                    return Ok(true);
                }
            }
        }

        Err(Error::new("Can't authenticate user"))
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
