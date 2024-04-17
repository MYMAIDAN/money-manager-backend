pub mod tracing;
pub(crate) mod repositories;
pub(crate) mod api;
pub(crate) mod middlewares;
pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod utils;
use config::Settings;
use crate::application::services::UserService;
use self::utils::AppState;

use super::infrastructure::repositories::user_repo_impl::UserRepositoryImpl;
use crate::infrastructure::api::routes::user_routes::create_user;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use sqlx::{postgres::PgPoolOptions, PgPool};


use axum::{
    routing::*,
    Router, extract::State, 
};

pub struct App{
    setting: Settings
}

impl App{
    pub fn new(_settings: Settings) -> Self{
        Self{setting:_settings}     
    }
}

pub async fn establish_connection(url: &str) -> Result<PgPool, sqlx::Error>{
    let pool  = PgPoolOptions::new()
    .max_connections(100)
    .connect(url).await?;
    Ok(pool)
}

pub async fn run_migration(pool: &PgPool) -> Result<(), sqlx::Error>{
    let _ = sqlx::migrate!("./migrations")
    .run(pool)
    .await?;
    Ok(())
}

impl App{
    pub async fn run(&self){
        dotenv().ok();

        let cors = tower_http::cors::CorsLayer::permissive();
        let pool = establish_connection(&self.setting.database.url)
        .await.expect("Database connection is not establish");
        run_migration(&pool).await;

        let pool = Arc::new(pool);
        let user_repository = Arc::new(UserRepositoryImpl::new(pool));
        let user_service = Arc::new(UserService::new(user_repository));
        let settings = self.setting.clone();

        let app_state =  utils::AppState  {
            user_service,
            settings
        };
        
        let app = Router::new()
            .route("/", get(|| async { "Hello, World" }))
            .route("/register", post(create_user))
            .layer(cors)
            .with_state(app_state);
            

        let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
        axum::serve(listener, app).await;
    }
}






