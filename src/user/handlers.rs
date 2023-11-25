use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tracing::info;

use crate::state::AppState;

use super::entity::CreateUser;

pub async fn create_new_user(
    State(state): State<Arc<AppState>>,
    Json(create_user): Json<CreateUser>,
) -> Response {
    info!("create user request: {:?}", create_user);
    let result =
        super::db::create_user(create_user, &state.pool, true, true, String::from("DAW")).await;

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
