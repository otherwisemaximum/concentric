use std::sync::Arc;
use std::time::Duration;

use axum::extract::State;
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
        .route("/login", post(crate::auth::handler::authenticate))
        .route(
            "/",
            get(crate::user::handlers::users).post(crate::user::handlers::create_new_user),
        )
        .layer(middleware)
        .with_state(state)
}
