//! OpenAPI specification and JSON endpoint.

use axum::{
    response::Response,
    http::{StatusCode, header}
};
use utoipa::OpenApi;
use serde_json;

use crate::constant::OPENAPI_CACHE_DURATION;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handler::health::health_check,
        crate::handler::request::handle_challenge_request,
        crate::handler::response::handle_challenge_response,
    ),
    components(
        schemas(
            ironshield_types::IronShieldRequest,
            ironshield_types::IronShieldChallenge,
            ironshield_types::IronShieldChallengeResponse,
            ironshield_types::IronShieldToken,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoint"),
        (name = "Challenge", description = "Challenge generation and verification endpoints")
    ),
    info(
        title = "IronShield API",
        version = env!("CARGO_PKG_VERSION"),
        description = "A stateless scraping & L7 DDoS protection solution optimized for performance, privacy, and accessibility",
        license(name = "", url = "")
    )
)]
pub struct ApiDoc;

/// Serves the OpenAPI JSON specification.
///
/// Returns a JSON response containing the complete OpenAPI specification
/// for the IronShield API, suitable for consumption by API documentation
/// tools and client generators.
pub async fn openapi_json() -> Response<axum::body::Body> {
    let openapi = ApiDoc::openapi();
    let json = serde_json::to_string_pretty(&openapi)
        .expect("Failed to serialize OpenAPI specification");

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::CACHE_CONTROL, format!("public, max-age={}", OPENAPI_CACHE_DURATION))
        .body(axum::body::Body::from(json))
        .expect("Failed to build OpenAPI JSON response")
}