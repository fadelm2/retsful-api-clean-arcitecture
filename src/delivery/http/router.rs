use crate::delivery::http::handler::user_handler::{login, register, AppState};
use axum::{routing::post, Router};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(app_state)
}
