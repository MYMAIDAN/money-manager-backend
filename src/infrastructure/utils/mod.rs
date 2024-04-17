use axum::extract::FromRef;

use crate::{application::services::UserService, domain::entity::user::User};

use super::{config::Settings, App};

#[derive(Debug, Clone)]
pub struct AppState{
    pub user_service: std::sync::Arc<UserService>,
    pub settings : Settings
}


impl FromRef<AppState> for std::sync::Arc<UserService>{
    fn from_ref(input: &AppState) -> Self {
        input.user_service.clone()
    }
}

impl FromRef<AppState> for Settings{
    fn from_ref(input: &AppState) -> Self {
        input.settings.clone()
    }
}
