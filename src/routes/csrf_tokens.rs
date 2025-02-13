use axum::{ extract::Path, http::StatusCode, Json, response::{IntoResponse, Response} };

use crate::db;


pub mod request {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Get {
        pub token: String
    }
}


pub mod response {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Post {
        pub token: String
    }
}


#[axum::debug_handler]
pub async fn get(Path(params): Path<request::Get>) -> Response {
    let Ok(connection) = &mut db::connection::establish() else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to connect to database"
        ).into_response();
    };

    // Check if user exists
    let Ok(valid_token) = db::queries::csrf_tokens::validate_csrf_token(connection, params.token) else {
        return (
            StatusCode::NOT_FOUND, 
            "Unable to get CSRF token"
        ).into_response()
    };

    if valid_token {
        StatusCode::OK.into_response()
    }
    else {
        (
            StatusCode::NOT_FOUND,
            "Invalid CSRF token"
        ).into_response()
    }
}


#[axum::debug_handler]
pub async fn post() -> Response {
    let Ok(connection) = &mut db::connection::establish() else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to connect to database"
        ).into_response();
    };

    // Check if user exists
    let Ok(csrf_token) = db::queries::csrf_tokens::create_csrf_token(connection) else {
        return (
            StatusCode::NOT_FOUND, 
            "Unable to create CSRF token"
        ).into_response()
    };

    (
        StatusCode::OK,
        Json(response::Post {
            token: csrf_token.uuid
        })
    ).into_response()
}