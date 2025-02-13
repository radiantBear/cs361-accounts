use axum::{ extract::Path, http::{HeaderMap, StatusCode}, Json, response::{IntoResponse, Response} };
use chrono::Utc;

use crate::db;


pub mod request {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Get {
        pub uuid: String
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
        pub user_id: i32
    }

    #[derive(Serialize)]
    pub struct Post {
        pub uuid: String
    }
}


pub async fn get(Path(params): Path<request::Get>) -> Response {
    let Ok(connection) = &mut db::connection::establish() else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to connect to database"
        ).into_response();
    };

    // Check if user exists
    let Ok((user, session)) = db::queries::users::get_user_from_session(connection, params.uuid) else {
        return (
            StatusCode::NOT_FOUND, 
            "Could not get user"
        ).into_response()
    };

    if Utc::now().naive_utc() > session.date_expires {
        return (
            StatusCode::UNAUTHORIZED, 
            "Session expired"
        ).into_response()
    }

    (
        StatusCode::OK,
        Json(response::Get {
            user_id: user.user_id
        })
    ).into_response()
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
    let Ok(user) = db::queries::users::get_user(connection, params.username, params.password) else {
        return (
            StatusCode::NOT_FOUND, 
            "Unable to find user"
        ).into_response()
    };

    // Create session for user
    let Ok(session) = db::queries::sessions::create_session(connection, user.user_id) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR, 
            "Unable to create session"
        ).into_response()
    };

    (
        StatusCode::OK,
        Json(response::Post {
            uuid: session.uuid
        })
    ).into_response()
}