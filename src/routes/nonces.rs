use axum::{ extract::Path, http::StatusCode, Json, response::{IntoResponse, Response} };

use crate::db;


pub mod request {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Delete {
        pub nonce: String
    }
}


pub mod response {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Post {
        pub nonce: String
    }
}


pub async fn post() -> Response {
    let Ok(connection) = &mut db::connection::establish() else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to connect to database"
        ).into_response();
    };

    // Check if user exists
    let Ok(nonce) = db::queries::nonces::create_nonce(connection) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR, 
            "Unable to create nonce"
        ).into_response()
    };

    (
        StatusCode::OK,
        Json(response::Post {
            nonce: nonce.uuid
        })
    ).into_response()
}


pub async fn delete(Path(params): Path<request::Delete>) -> Response {
    let Ok(connection) = &mut db::connection::establish() else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to connect to database"
        ).into_response();
    };

    // Check if user exists
    let Ok(valid_token) = db::queries::nonces::validate_nonce(connection, params.nonce) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR, 
            "Unable to check nonce"
        ).into_response()
    };

    if valid_token {
        StatusCode::OK.into_response()
    }
    else {
        (
            StatusCode::NOT_FOUND,
            "Invalid nonce"
        ).into_response()
    }
}
