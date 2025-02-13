use axum::{ extract::Path, http::StatusCode, response::{IntoResponse, Response}, Json };
use diesel::result::{DatabaseErrorKind, Error as DieselError};

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


pub async fn post(Json(params): Json<request::Post>) -> Response {
        let Ok(connection) = &mut db::connection::establish() else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to connect to database"
        ).into_response();
    };

    // Check if user exists
    match db::queries::users::create_user(connection, params.username, params.password) {
        Ok(user) => (StatusCode::OK, Json(response::Post { id: user.user_id })).into_response(),
        Err(e) => if let db::types::Error::DieselError(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) = e {
                ( StatusCode::CONFLICT, "Username already in use" ).into_response()
            } else {
                ( StatusCode::INTERNAL_SERVER_ERROR, "Unable to save user" ).into_response()
            }
    }
}


pub async fn delete(Path(user_id): Path<i32>) -> Response {
        let Ok(connection) = &mut db::connection::establish() else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to connect to database"
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
