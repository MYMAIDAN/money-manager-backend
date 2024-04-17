use core::fmt;

use async_trait::async_trait;
use thiserror::Error;
use crate::domain::{entity::user::User, value_object::Email};

#[derive(Error,Debug)]
pub enum CreateUserError{
    #[error("Connection problem")]
    Connection
}

#[derive(Error,Debug)]
pub enum GetUserError{
    #[error("User not fount")]
    NotFound,
    #[error("Connection problem")]
    Connection
}

#[async_trait]
pub trait UserRepository{
    async fn create_user(&self, user: &User) -> Result<(),CreateUserError>;
    async fn get_by_email(&self, email: &Email) -> Result<User, GetUserError>;
    async fn get_by_id(&self, id: u64 ) -> Result<User, GetUserError>;
}
