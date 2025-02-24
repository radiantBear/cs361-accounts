use axum::{ http::StatusCode, Json, response::{IntoResponse, Response} };

use crate::utils::rand;


pub mod response {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Get {
        pub token: String
    }
}


pub async fn get() -> Response {
    let uuid = rand::generate_alphanumeric(128);

    (
        StatusCode::OK,
        Json(response::Get {
            token: uuid
        })
    ).into_response()
}
