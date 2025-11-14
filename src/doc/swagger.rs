//! Swagger UI interface for API documentation.

use axum::{
    response::Response,
    http::{StatusCode, header}
};

use crate::constant::{SWAGGER_UI_CACHE_DURATION, SWAGGER_UI_HTML};

/// Serves a custom Swagger UI HTML page with IronShield branding.
///
/// Provides an interactive API documentation interface using Swagger UI,
/// styled with IronShield's brand colors and configured to load the
/// OpenAPI specification from the `/api-docs/openapi.json` endpoint.
pub async fn swagger_ui() -> Response<axum::body::Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .header(header::CACHE_CONTROL, format!("public, max-age={}", SWAGGER_UI_CACHE_DURATION))
        .body(axum::body::Body::from(SWAGGER_UI_HTML))
        .expect("Failed to build Swagger UI response")
}