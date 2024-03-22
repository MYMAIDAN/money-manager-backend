use axum::{
    extract::{Request,Json},
    routing::post, http::{Response, StatusCode}, response::IntoResponse,
};

use axum::extract::State;
use super::{service::UserService, errors::UserCreateError};

use super::{dtos::UserDTO, repository::UserRepository};


pub async fn create_user(
    State(state): State<std::sync::Arc<UserService>>,
    Json(payload): Json<UserDTO>) -> Result<StatusCode, UserCreateError> {
        state.create_user(payload).await?;
        Ok(StatusCode::CREATED)
    }


