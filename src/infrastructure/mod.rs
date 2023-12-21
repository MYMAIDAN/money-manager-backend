pub mod database;
pub mod tracing;
use crate::application::user::service::UserService;
use crate::adapters::db::user::UserRepositoryImpl;
use crate::application::user::routes::create_user;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

use axum::{
    routing::*,
    Router, extract::State, 
};

pub struct App{}

impl App{
    pub fn new() -> Self{
        Self{}
    }
}

impl App{
    pub async fn run(&self){
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = crate::adapters::db::establish_connection(database_url.as_str())
        .await.expect("Database connection is not establish");
        crate::adapters::db::run_migration(&pool).await;

        let pool = Arc::new(pool);
        let user_repository = Arc::new(UserRepositoryImpl::new(pool));
        let user_service = Arc::new(UserService::new(user_repository));

        let app = Router::new()
            .route("/", get(|| async { "Hello, World" }))
            .route("/register", post(create_user))
            .with_state(user_service);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
        axum::serve(listener, app).await;
    }
}




