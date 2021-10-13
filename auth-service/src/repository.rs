use diesel::prelude::*;

use crate::models::{NewUser, User};
use crate::schema::users;

pub fn get_user_by_id(id: i32, conn: &PgConnection) -> QueryResult<User> {
    users::table.filter(users::id.eq(id)).first(conn)
}

pub fn get_user_by_email(email: &str, conn: &PgConnection) -> QueryResult<User> {
    users::table.filter(users::email.eq(email)).first(conn)
}

pub fn create_user(new_user: NewUser, conn: &PgConnection) -> QueryResult<User> {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users).values(new_user).get_result(conn)
}
