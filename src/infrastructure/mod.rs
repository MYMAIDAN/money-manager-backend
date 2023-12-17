pub mod database;
pub mod tracing;

use crate::application::user::create_user::create_user;

use axum::{
    routing::*,
    Router
};

pub struct App{}

impl App{
    pub fn new() -> Self{
        Self{}
    }
}

impl App{
    pub async fn run(&self){
        let app = Router::new()
            .route("/", get(|| async { "Hello, World" }))
            .route("/register", post(create_user));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
        axum::serve(listener, app).await;
    }
}




