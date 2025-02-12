use axum::{ extract::Query, http::StatusCode, Json, response::{IntoResponse, Response} };

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


#[axum::debug_handler]
pub async fn get(Query(params): Query<request::Get>) -> Response {
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
            "Could not get user"
        ).into_response()
    };

    (
        StatusCode::OK,
        Json(response::Get {
            username: user.username, 
            date_created: user.date_created,
            date_updated: user.date_updated
        })
    ).into_response()
}


#[axum::debug_handler]
pub async fn post(Query(params): Query<request::Post>) -> Response {
    let Ok(connection) = &mut db::connection::establish() else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to connect to database"
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