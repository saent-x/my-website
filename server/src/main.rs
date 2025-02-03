#![allow(unused)] // temporary measure
mod db;
mod error;
mod handlers;
mod models;
mod prelude;
mod util;

use std::{env, sync::Arc};

use axum::{ http::{ header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method}, response::{Html, IntoResponse}, routing::{get, post}, Router};
use db::Database;
use dotenv::dotenv;
use handlers::{blog_handler::{create_blog_post, delete_blog_post_by_id, get_blog_post_by_id, get_blog_posts, get_latest_posts, get_total_posts_count, update_blog_post_by_id}, category_handler::{create_category, delete_category_by_id, get_categories, get_category_by_id, get_total_categories_count}, health_check, message_handler::{create_message, delete_message_by_id, get_all_messages, get_message_by_id, get_total_messages_count, update_message_by_id}, website_info_handler::get_website_info};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!(
                "{}=debug,tower_http=debug,axum::rejection=trace",
                env!("CARGO_CRATE_NAME")
            )
            .into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    let db = Database::init().await.expect("failed to connect to db");
    let ui_url = env::var("UI_URL").expect("cannot find variable");
    
    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(ui_url.parse::<HeaderValue>().unwrap())
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
        .route("/api/health", get(health_check))
        .route("/api/blog", post(create_blog_post).get(get_blog_posts))
        .route("/api/blog/:id", get(get_blog_post_by_id).delete(delete_blog_post_by_id).post(update_blog_post_by_id))
        .route("/api/blog/count", get(get_total_posts_count))
        .route("/api/blog/latest_posts", get(get_latest_posts))
        .route("/api/category", post(create_category).get(get_categories))
        .route("/api/category/:id", get(get_category_by_id).delete(delete_category_by_id))
        .route("/api/category/count", get(get_total_categories_count))
        .route("/api/messages", post(create_message).get(get_all_messages))
        .route("/api/messages/:id", get(get_message_by_id).delete(delete_message_by_id).post(update_message_by_id))
        .route("/api/messages/count", get(get_total_messages_count))
        .route("/api/website_info", get(get_website_info))
        .with_state(app_state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
