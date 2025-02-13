pub mod db;
pub mod routes;


use axum::{ routing::{get, post}, Router };


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", post(routes::users::post))
        .route("/sessions", post(routes::sessions::post))
        .route("/sessions/{uuid}", get(routes::sessions::get))
        .route("/csrf_tokens", post(routes::csrf_tokens::post))
        .route("/csrf_tokens/{token}", get(routes::csrf_tokens::get));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
