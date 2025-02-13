pub mod db;
pub mod routes;
pub mod utils;


#[tokio::main]
async fn main() {
    let app = routes::app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
