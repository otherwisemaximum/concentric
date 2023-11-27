use std::sync::Arc;
use std::time::Duration;

use axum::extract::{State, TypedHeader};
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::post;
use axum::{middleware, Json};
use axum::{routing::get, Router};
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};
use tower_http::CompressionLevel::Fastest;
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    timeout::{RequestBodyTimeoutLayer, TimeoutLayer},
};

use crate::state::AppState;

async fn server_name_header<B>(request: Request<B>, next: Next<B>) -> Response {
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .append("X-Server", "DAW".parse().unwrap());
    response
}

async fn _auth<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    dbg!(auth);
    let response = next.run(request).await;
    response
}

pub fn build_routes(state: Arc<AppState>) -> Router {
    let middleware = tower::ServiceBuilder::new()
        .layer(CompressionLayer::new().quality(Fastest))
        .layer(RequestBodyTimeoutLayer::new(Duration::from_secs(30)))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
        .layer(middleware::from_fn(server_name_header))
        .layer(CatchPanicLayer::new());

    Router::new()
        .route(
            "/api/users",
            get(crate::user::handlers::users).post(crate::user::handlers::create_new_user),
        )
        // no auth needed beyond this point
        .route("/auth/login", post(crate::auth::handler::authenticate))
        .route(
            "/health",
            get(|State(state): State<Arc<AppState>>| async move {
                tracing::info!("up since {} minute(s)", state.uptime());
                (
                    StatusCode::OK,
                    Json(json!({
                        "code": 200,
                        "uptime": state.uptime(),
                        "health": "healthy"
                    })),
                )
            }),
        )
        .layer(middleware)
        .with_state(state)
}
