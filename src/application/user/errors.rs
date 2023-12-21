use sqlx::error;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum UserCreateError{
    #[error("User already exist.")]
    AlreadyExist,
    #[error("Name is empty.")]
    NameIsEmpty,
    #[error("Surname is empty.")]
    SurnameIsEmpty,
    

    
}