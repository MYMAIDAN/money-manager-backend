use axum::{
    extract::{Request,Json},
    routing::post, http::{Response, StatusCode}, response::IntoResponse,
};

use axum::extract::State;
use crate::application::services::{UserService, UserCreateError};

use crate::application::dtos::user::UserDTO;
use crate::application::repositories::UserRepository;
use crate::infrastructure::error::Error;


pub async fn create_user(
    State(state): State<std::sync::Arc<UserService>>,
    Json(payload): Json<UserDTO>) -> Result<StatusCode, UserCreateError> {
        state.create_user(payload).await?;
        Ok(StatusCode::CREATED)
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

