use std::net::SocketAddr;
use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;
use sqlx::mysql::MySqlPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod state;
mod user;

use state::AppState;

async fn health(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    state.uptime();
    (StatusCode::OK, Json(json!({"health": "healthy"})))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = "mysql://root:root@localhost/concentricdev";

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .unwrap();

    let state = Arc::new(AppState::new(pool));

    let _cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let app = Router::new()
        .route("/health", get(health))
        .nest("/api/user", user::user_routes_service())
        .nest("/auth", auth::auth_route_service())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
