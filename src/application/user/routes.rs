use axum::{
    extract::{Request,Json},
    routing::post, http::Response,
};

use axum::extract::State;
use super::service::UserService;

use super::{dto::UserDTO, repository::UserRepository};


pub async fn create_user(
    State(state): State<std::sync::Arc<UserService>>,
    Json(payload): Json<UserDTO>){
        state.create_user(payload).await;
}

