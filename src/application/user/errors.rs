use axum::{response::IntoResponse, http::StatusCode, Json};
use serde::Serialize;
use sqlx::error;
use thiserror::Error;

use crate::domain::{value_object::PasswordError, entity::user::User};


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

#[derive(Serialize)]
struct Error{
    error_code: String,
    message: String
}

impl Error{
    fn new(code: &str, message: &str) -> Self{
        Self { error_code: code.to_owned(), message: message.to_owned() }
    }
}


impl IntoResponse for UserCreateError{
    fn into_response(self) -> axum::response::Response {
        let body = match self{
            Self::AlreadyExist => Error::new("USER_ALREADY_EXIST", self.to_string().as_str()),
            Self::NameIsEmpty =>Error::new("NAME_FIELD_IS_EMPTY", self.to_string().as_str()),
            Self::SurnameIsEmpty => Error::new("SURNAME_FIELD_IS_EMPTY",self.to_string().as_str()),
            Self::PasswordIncorect(_) => Error::new("PASSWORD_FILE_IS_INCORRECT", self.to_string().as_str())
        };

        (StatusCode::BAD_REQUEST, Json(body)).into_response()
    }
}