use axum::{extract::Request, http::{HeaderMap, StatusCode}, middleware::Next, response::{IntoResponse, Response}};

use crate::config::CONFIG;


pub async fn validate_api_key(headers: HeaderMap, req: Request, next: Next) -> Result<Response, StatusCode> {    
    println!("Handling {}\t{}", req.method(), req.uri().path());
    
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
