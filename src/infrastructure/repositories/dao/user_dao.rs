use uuid::Uuid;

use crate::{domain};

#[derive(Debug,Clone)]
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