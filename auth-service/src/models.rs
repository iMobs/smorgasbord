use crate::schema::users;

#[derive(Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
}
