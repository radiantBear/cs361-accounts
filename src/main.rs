pub mod config;
pub mod db;
pub mod middleware;
pub mod routes;
pub mod utils;


#[tokio::main]
async fn main() {
    if !config::load_config() {
        return;
    }

    let app = routes::app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}