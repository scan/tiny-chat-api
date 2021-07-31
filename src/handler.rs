use std::{collections::HashMap, convert::Infallible};
use warp::http::StatusCode;

pub async fn health() -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::NO_CONTENT)
}

pub async fn not_found() -> Result<impl warp::Reply, Infallible> {
    let mut not_found_response = HashMap::new();
    not_found_response.insert("error", "not found");

    Ok(warp::reply::with_status(
        warp::reply::json(&not_found_response),
        StatusCode::NOT_FOUND,
    ))
}
