use std::sync::Arc;
use crate::domain::entity::user::User;
use crate::domain::value_object::*;
use crate::application::dtos::user::UserDTO;
use crate::application::repositories::UserRepository;
use axum::Error;
use serde::Serialize;
use thiserror::Error;
use crate::domain::value_object::PasswordError;
use uuid::{Uuid, Timestamp, NoContext};
use std::marker::{Send,Sync};

#[derive(Clone)]
pub struct UserService{
    repository: Arc<dyn UserRepository + Send + Sync>
}


impl UserService{
    pub fn new(repository: Arc<dyn UserRepository + Send + Sync>) -> Self{
        Self { repository: repository.clone() }
    }

    pub async fn create_user(&self, new_user: UserDTO ) -> Result<(), UserCreateError>{

        let password = Password::try_from(new_user.password)?;
        let email = Email::from(new_user.email.clone());
        match  self.repository.get_by_email(&email).await {
            Ok(_) => Err(UserCreateError::AlreadyExist),
            Err(_) => {
            let user = User::new(
                        ID::from(Uuid::new_v7(Timestamp::now(NoContext))),
                        Name::from(new_user.name),
                        Surname::from(new_user.surname),
                        Email::from(new_user.email),
                        password
                );
        
                let create_user = self.repository.create_user(&user).await;
            
                Ok(())
            }

        }
    }

    
    
}

#[derive(Error, Debug)]
pub enum UserCreateError{
    #[error("User already exist.")]
    AlreadyExist,
    #[error("Name is empty.")]
    NameIsEmpty,
    #[error("Surname is empty.")]
    SurnameIsEmpty,
    #[error("Password is incorect.")]
    PasswordIncorect( PasswordError  )
}

impl From<PasswordError> for UserCreateError{
    fn from(value: PasswordError) -> Self {
        UserCreateError::PasswordIncorect(value)
    }
}

