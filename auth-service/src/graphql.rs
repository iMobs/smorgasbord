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

    async fn get_user(&self, ctx: &Context<'_>, id: ID) -> Result<User> {
        get_user_internal(ctx, id)
    }

    #[graphql(entity)]
    async fn find_user_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<User> {
        get_user_internal(ctx, id)
    }
}

fn get_user_internal(ctx: &Context<'_>, id: ID) -> Result<User> {
    let id = id.parse()?;
    let user = crate::repository::get_user_by_id(id, &get_conn_from_ctx(ctx))?;
    Ok(User::from(user))
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn signup(&self, ctx: &Context<'_>, user: UserInput) -> Result<User> {
        let user = NewUserEntity {
            email: user.email,
            password: hash_password(&user.password)?,
        };

        let user = crate::repository::create_user(user, &get_conn_from_ctx(ctx))?;

        Ok(User::from(user))
    }

    async fn login(&self, ctx: &Context<'_>, email: String, password: String) -> Result<User> {
        let maybe_user = crate::repository::get_user_by_email(&email, &get_conn_from_ctx(ctx)).ok();

        if let Some(user) = maybe_user {
            if let Ok(matching) = verify_password(&user.password, &password) {
                if matching {
                    return Ok(User::from(user));
                }
            }
        }

        Err(Error::new("Can't authenticate user"))
    }
}

#[derive(SimpleObject)]
struct User {
    id: ID,
    email: String,
}

#[derive(InputObject)]
struct UserInput {
    email: String,
    password: String,
}

impl From<UserEntity> for User {
    fn from(entity: UserEntity) -> Self {
        User {
            id: ID::from(entity.id),
            email: entity.email,
        }
    }
}
