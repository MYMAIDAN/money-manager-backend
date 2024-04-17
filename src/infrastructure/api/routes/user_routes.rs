use axum::{
    extract::Json, http::{header, Response, StatusCode}, response::IntoResponse 
};

use serde_json::*;
use axum::extract::State;
use axum_extra::extract::cookie::{self, Cookie};
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::{application::services::{UserCreateError, UserService}, infrastructure::config::Settings};
use crate::infrastructure::middlewares::auth::create_token;

use crate::application::dtos::user::UserDTO;
use crate::infrastructure::error::Error;


pub async fn create_user(
    State(state): State<std::sync::Arc<UserService>>,
    State(settings): State<Settings>,
    Json(payload): Json<UserDTO>) -> std::result::Result<impl IntoResponse, UserCreateError> {
        let user_id = state.create_user(payload).await?;
                
        let token_claim =  create_token(user_id);
        let token = encode(
            &Header::default(),
            &token_claim,
            &EncodingKey::from_secret(settings.jwt.secret.as_ref())
        ).unwrap();

        let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(cookie::SameSite::Lax)
        .http_only(true);
        

        let mut response = Response::new(json!({"status": "success", "token": token}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
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

