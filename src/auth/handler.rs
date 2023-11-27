use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{EncodingKey, Header};
use serde_json::json;
use tracing::info;

use crate::entity::auth::{AuthorizeUser, Claims};
use crate::{state::AppState, user::queries::get_user_by_email};

pub async fn authenticate(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthorizeUser>,
) -> impl IntoResponse {
    let user = get_user_by_email(&payload.email.as_str(), &state.pool).await;
    info!("user: {:?}", user);
    if user.is_err() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "code": 400,
                "message": "Invalid credentials"
            })),
        );
    }

    let user = user.unwrap();
    let valid_password = bcrypt::verify(payload.password, user.password.as_str());
    info!("valid_password: {:?}", valid_password);
    if valid_password.is_err() {
        info!("seems like we got bcrypt error: {:?}", valid_password.err());
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "code": 400,
                "message": "Invalid credentials"
            })),
        );
    }

    let token = jsonwebtoken::encode(
        &Header::default(),
        &Claims {
            username: user.username,
        },
        &EncodingKey::from_secret("secret".as_bytes()),
    );
    info!("token: {:?}", token);

    if token.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "code": 500,
                "message": "Server error while authenticating user"
            })),
        );
    }

    return (
        StatusCode::OK,
        Json(json!({
            "code": 200,
            "iss": "https://iam.concentric.app",
            "token": token.unwrap()
        })),
    );
}
