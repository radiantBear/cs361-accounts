pub mod csrf_tokens;
pub mod sessions;
pub mod users;


use axum::{Router, middleware, routing::{get, post, delete}};

use crate::middleware::{validate_api_key, validate_csrf_token};


pub fn app() -> Router {
    Router::new()
        .nest("/users", 
            Router::new()
            .route("/", post(users::post))
            .route("/{user_id}", delete(users::delete))
            .layer(middleware::from_fn(validate_csrf_token))
        )
        
        .nest("/sessions",
            Router::new()
            .route("/", post(sessions::post))
            .layer(middleware::from_fn(validate_csrf_token))
        )
        .route("/sessions/{uuid}",     get(sessions::get))
        
        .route("/csrf_tokens",         post(csrf_tokens::post))
        .route("/csrf_tokens/{token}", get(csrf_tokens::get))
        
        .layer(middleware::from_fn(validate_api_key))
}
