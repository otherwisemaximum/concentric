use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use serde_json::json;
use tracing::info;

use crate::{entity, state::AppState};

use crate::entity::user::CreateUser;

pub async fn create_new_user(
    State(state): State<Arc<AppState>>,
    Json(create_user): Json<CreateUser>,
) -> Response {
    info!("create user request: {:?}", create_user);
    let result =
        super::queries::create_user(create_user, &state.pool, true, true, String::from("DAW"))
            .await;

    if result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "code": 500,
                "message": "error creating user"
            })),
        )
            .into_response();
    }

    (StatusCode::CREATED).into_response()
}

pub async fn users() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(vec![entity::user::User {
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
