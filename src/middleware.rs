use axum::{extract::Request, http::{HeaderMap, StatusCode}, middleware::Next, response::{IntoResponse, Response}};

use crate::config::CONFIG;
use crate::db;


pub async fn validate_api_key(headers: HeaderMap, req: Request, next: Next) -> Result<Response, StatusCode> {    
    let api_key = headers
        .get("x-api-key")
        .and_then(|header| header.to_str().ok())
        .map(|key| key.to_string());

    let Some(api_key) = api_key else {
        return Ok((
            StatusCode::BAD_REQUEST, 
            "Unable to parse API key".to_string()
        ).into_response());
    };
    
    if api_key != CONFIG.get().unwrap().api_key {
        return Ok((
            StatusCode::UNAUTHORIZED,
            "Invalid API key"
        ).into_response())
    };

    Ok(next.run(req).await)
}


pub async fn validate_csrf_token(headers: HeaderMap, req: Request, next: Next) -> Result<Response, StatusCode> {
    let csrf_token = headers
        .get("x-csrf-token")
        .and_then(|header| header.to_str().ok())
        .map(|token| token.to_string());

    let Some(csrf_token) = csrf_token else {
        return Ok((
            StatusCode::BAD_REQUEST, 
            "Unable to parse CSRF token".to_string()
        ).into_response());
    };

    let Ok(conn) = &mut db::connection::establish() else {
        return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to connect to database"
        ).into_response());
    };
    
    let Ok(true) = db::queries::csrf_tokens::validate_csrf_token(conn, csrf_token) else {
        return Ok((
            StatusCode::FORBIDDEN,
            "Invalid CSRF token"
        ).into_response())
    };

    Ok(next.run(req).await)
}
