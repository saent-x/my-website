#![allow(unused)] // temporary measure
mod db;
mod error;
mod handlers;
mod models;
mod util;
use std::sync::Arc;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use db::Database;
use dotenv::dotenv;
use handlers::blog_handler::{create_blog_post, health_check};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    dotenv().ok(); // load dotenv

    // initialize db
    let db = Database::init().await.expect("failed to connect to db");

    let cors: CorsLayer = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("[INFO] server started...");
    axum::serve(tcp_listener, load_router(Arc::new(db), cors).await)
        .await
        .expect("[ERROR] error ocurred while starting server");

    Ok(())
}

async fn load_router(app_state: Arc<Database>, cors: CorsLayer) -> Router {
    Router::new()
        .route("/api/hello", get(hello_server))
        .route("/api/health", get(health_check))
        .route("/api/blog/create", post(create_blog_post))
        .with_state(app_state)
        .layer(cors)
}

async fn hello_server() -> impl IntoResponse {
    Html("<h1>Hello! Server...</h1>")
}

// temporary fn to preload blog posts
async fn load_db() {}