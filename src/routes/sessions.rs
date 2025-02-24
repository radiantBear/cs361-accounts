use axum::{ extract::Path, http::StatusCode, Json, response::{IntoResponse, Response} };
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
        pub id: i32
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
            StatusCode::NOT_FOUND, 
            "Session expired"
        ).into_response()
    }

    (
        StatusCode::OK,
        Json(response::Get {
            id: user.user_id
        })
    ).into_response()
}


pub async fn post(Json(params): Json<request::Post>) -> Response {
        let Ok(connection) = &mut db::connection::establish() else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to connect to database"
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