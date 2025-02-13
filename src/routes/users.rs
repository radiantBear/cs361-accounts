use axum::{ extract::Path, http::{HeaderMap, StatusCode}, response::{IntoResponse, Response}, Json };

use crate::db;


pub mod request {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Get {
        pub username: String,
        pub password: String
    }

    #[derive(Deserialize)]
    pub struct Post {
        pub username: String,
        pub password: String
    }
}


pub mod response {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Get {
        pub username: String,
        pub date_created: chrono::NaiveDateTime,
        pub date_updated: chrono::NaiveDateTime
    }

    #[derive(Serialize)]
    pub struct Post {
        pub id: i32
    }
}


pub async fn post(headers: HeaderMap, Json(params): Json<request::Post>) -> Response {
    let csrf_token = headers
        .get("x-csrf-token")
        .and_then(|header| header.to_str().ok())
        .map(|token| token.to_string());

    let Some(csrf_token) = csrf_token else {
        return (
            StatusCode::BAD_REQUEST, 
            "Unable to parse CSRF token"
        ).into_response();
    };

    let Ok(connection) = &mut db::connection::establish() else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to connect to database"
        ).into_response();
    };

    // Check that CSRF token is valid
    let Ok(true) = db::queries::csrf_tokens::validate_csrf_token(connection, csrf_token) else {
        return (
            StatusCode::FORBIDDEN,
            "Invalid CSRF token"
        ).into_response();
    };

    // Check if user exists
    let Ok(user) = db::queries::users::create_user(connection, params.username, params.password) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR, 
            "Unable to save user"
        ).into_response()
    };

    (
        StatusCode::OK,
        Json(response::Post {
            id: user.user_id
        })
    ).into_response()
}


pub async fn delete(Path(user_id): Path<i32>, headers: HeaderMap) -> Response {
    let csrf_token = headers
        .get("x-csrf-token")
        .and_then(|header| header.to_str().ok())
        .map(|token| token.to_string());

    let Some(csrf_token) = csrf_token else {
        return (
            StatusCode::BAD_REQUEST, 
            "Unable to parse CSRF token"
        ).into_response();
    };
    
    let Ok(connection) = &mut db::connection::establish() else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to connect to database"
        ).into_response();
    };

    // Check that CSRF token is valid
    let Ok(true) = db::queries::csrf_tokens::validate_csrf_token(connection, csrf_token) else {
        return (
            StatusCode::FORBIDDEN,
            "Invalid CSRF token"
        ).into_response();
    };
    
    // Delete user
    let Ok(_) = db::queries::users::delete_user(connection, user_id) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to delete user"
        ).into_response();
    };

    StatusCode::OK.into_response()
}
