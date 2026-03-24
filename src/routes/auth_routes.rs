use axum::{Router, routing::post};
use crate::app_state::AppState;
use crate::handlers::auth_handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login))
}
