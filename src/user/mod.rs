use std::sync::Arc;

use axum::{debug_handler, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use chrono::Utc;

use crate::state::AppState;

pub mod db;
pub mod entity;
mod handlers;

pub fn user_routes_service() -> Router<Arc<AppState>> {
    Router::new().route("/", get(users).post(handlers::create_new_user))
}

#[debug_handler]
pub async fn users() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(vec![entity::User {
            id: 1,
            username: String::from(""),
            email: String::from(""),
            password: String::from(""),
            active: true,
            internal_flag: true,
            create_timestamp: Utc::now(),
            create_user: String::from(""),
            update_timestamp: Utc::now(),
            update_user: String::from(""),
        }]),
    )
}
