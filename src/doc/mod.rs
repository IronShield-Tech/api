//! Documentation and API interface modules.
//!
//! This module contains all documentation-related functionality including
//! OpenAPI specification, Swagger UI, and static assets.

mod openapi;
mod swagger;
mod assets;

pub use openapi::{openapi_json, ApiDoc};
pub use swagger::swagger_ui;
pub use assets::favicon;