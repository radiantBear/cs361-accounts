pub mod csrf_tokens;
pub mod sessions;
pub mod users;


use axum::{Router, routing::{get, post}};


pub fn app() -> Router {
    Router::new()
        .route("/users", post(self::users::post))
        .route("/sessions", post(self::sessions::post))
        .route("/sessions/{uuid}", get(self::sessions::get))
        .route("/csrf_tokens", post(self::csrf_tokens::post))
        .route("/csrf_tokens/{token}", get(self::csrf_tokens::get))
}