pub mod db;
pub mod models;
pub mod schema;

use axum::{ extract::Query, http::StatusCode, response::{IntoResponse, Response}, routing::get, Json, Router };
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
struct ReqGetUser {
    username: String,
    password: String
}


#[derive(Serialize)]
struct RespGetUser {
    pub username: String,
    pub date_created: chrono::NaiveDateTime,
    pub date_updated: chrono::NaiveDateTime
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", get(get_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[axum::debug_handler]
async fn get_user(Query(params): Query<ReqGetUser>) -> Response {
    let Ok(connection) = &mut db::establish_connection() else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to connect to database"
        ).into_response();
    };

    // Check if user exists
    let Ok(user) = db::get_user(connection, params.username.as_str(), params.password.as_str()) else {
        return (
            StatusCode::NOT_FOUND, 
            "Could not get user"
        ).into_response()
    };

    (
        StatusCode::OK,
        Json(RespGetUser {
            username: user.username, 
            date_created: user.date_created,
            date_updated: user.date_updated
        })
    ).into_response()
}