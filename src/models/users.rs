use diesel::{Insertable, Queryable};
use uuid::Uuid;

use super::super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: Uuid,
    pub username: &'a str,
    pub password: &'a str,
}
