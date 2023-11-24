use std::sync::Arc;
use std::{env, net::SocketAddr};

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;
use sqlx::mysql::MySqlPoolOptions;

mod context;
mod user;

use context::AppState;

async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"health": "healthy"})))
}

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "daw");
    env_logger::init();

    let db_url = "mysql://root:root@localhost/concentricdev";

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .unwrap();

    let state = Arc::new(AppState::new(pool));

    let app = Router::new()
        .route("/health", get(health))
        .nest("/user", user::user_layer())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
