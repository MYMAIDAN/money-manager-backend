use serde::de::IntoDeserializer;
use uuid::Uuid;

use crate::domain::{self, entity::user::User, value_object::{Surname, ID}};

#[derive(Debug,Clone)]
#[derive(sqlx::FromRow)]
pub struct UserDaoModel{
    pub id: Uuid,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String
}


impl From<domain::entity::user::User> for UserDaoModel{
    fn from(value: domain::entity::user::User) -> Self {
        Self { 
            id: Uuid::from(value.id()), 
            name: String::from(value.name()), 
            surname: String::from(value.surname()), 
            email: String::from(value.email()), 
            password: String::from(value.password()) 
        }
    }
}


impl From<&domain::entity::user::User> for UserDaoModel{
    fn from(value: &domain::entity::user::User) -> Self {
        Self { 
            id: Uuid::from(value.id()), 
            name: String::from(value.name()), 
            surname: String::from(value.surname()), 
            email: String::from(value.email()), 
            password: String::from(value.password()) 
        }
    }
}

impl From<UserDaoModel> for User{
    fn from(value: UserDaoModel) -> Self {
        User::new(
            value.id.into(), 
            value.name.into(), 
            value.surname.into(),
             value.email.into(), 
             value.password.try_into().unwrap())
    }
}