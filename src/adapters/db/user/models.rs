use diesel::deserialize::{Queryable, QueryableByName};
use uuid::Uuid;

use crate::schema::users;

#[derive(Queryable,QueryableByName)]
#[table_name = "users"]
pub struct User{
    pub id: Uuid,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String
     
}