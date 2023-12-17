use axum::{
    extract::{Request,Json},
    routing::post
};

use super::dto::User;


pub async fn create_user(Json(payload): Json<User>) {
    tracing::trace!("User has been registerd: {:?}", payload);        
}

