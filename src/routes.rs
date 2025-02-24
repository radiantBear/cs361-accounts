pub mod csrf_tokens;
pub mod nonces;
pub mod sessions;
pub mod users;


use axum::{Router, middleware, routing::{get, post, delete}};

use crate::middleware::validate_api_key;


pub fn app() -> Router {
    Router::new()
        .route("/users", post(users::post))
        .route("/users/{user_id}", delete(users::delete))
        
        .route("/sessions", post(sessions::post))
        .route("/sessions/{uuid}", get(sessions::get))
        
        .route("/csrf_tokens", get(csrf_tokens::get))

        .route("/nonces", post(nonces::post))
        .route("/nonces/{nonce}", delete(nonces::delete))
        
        .layer(middleware::from_fn(validate_api_key))
}
