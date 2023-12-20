use axum::{
    extract::{Request,Json},
    routing::post, Extension
};

use super::{dto::User, repository::UserRepository};


pub async fn create_user(Json(payload): Json<User>, Extension(UserRepository): Extension<impl UserRepository>) {
    tracing::trace!("User has been registerd: {:?}", payload);        
}

