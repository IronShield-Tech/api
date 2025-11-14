//! Static assets for the API documentation.

use axum::{
    response::Response,
    http::{StatusCode, header}
};

use crate::constant::FAVICON_CACHE_DURATION;

/// Serves the favicon.ico file.
///
/// The favicon is embedded at compile time to ensure availability in
/// production environments without requiring external file system access.
pub async fn favicon() -> Response<axum::body::Body> {
    const FAVICON_DATA: &[u8] = include_bytes!("../../assets/favicon.ico");

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/x-icon")
        .header(header::CACHE_CONTROL, format!("public, max-age={}", FAVICON_CACHE_DURATION))
        .body(axum::body::Body::from(FAVICON_DATA))
        .expect("Failed to build favicon response")
}