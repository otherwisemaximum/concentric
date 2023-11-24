use std::sync::Arc;

use axum::{debug_handler, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use chrono::Utc;

use crate::context::AppState;

pub mod entity;

pub fn user_layer() -> Router<Arc<AppState>> {
    Router::new().route("/", get(users))
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
        }]),
    )
}
