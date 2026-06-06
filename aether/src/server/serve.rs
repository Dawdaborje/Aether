use aether_core::config_manager::models;
use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};

pub async fn run_server(server: &str, config_file: models::AetherConfig) {
    let app = Router::new();
    println!(
        "Starting server on {server} and using configuration file: {:?}",
        config_file
    );
    let listener = tokio::net::TcpListener::bind(server).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
