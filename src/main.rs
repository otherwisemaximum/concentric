use std::net::SocketAddr;
use std::sync::Arc;

use sqlx::mysql::MySqlPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod entity;
mod router;
mod state;
mod user;

use state::AppState;

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

    let app = router::build_routes(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
