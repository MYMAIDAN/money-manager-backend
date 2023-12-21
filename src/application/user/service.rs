use std::sync::Arc;
use crate::domain::entity::user::User;
use crate::domain::value_object::*;
use super::repository::*;
use super::dto::UserDTO;
use super::errors::UserCreateError;
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