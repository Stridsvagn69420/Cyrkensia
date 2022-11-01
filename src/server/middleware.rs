use actix_web::middleware::DefaultHeaders;
use crate::meta;

/// License Headers
/// 
/// Appends the license headers (`X-License`, `X-License-SPDX` and `X-License-URL`) to every HTTP Response.
pub fn license_headers() -> DefaultHeaders {
    DefaultHeaders::new()
    .add(("X-License", meta::LICENSE_RICH))
    .add(("X-License-SPDX", meta::LICENSE))
    .add(("X-License-URL", meta::LICENSE_URL))
}

/// Owner and Source Code Headers
/// 
/// Appends the `X-Authors` and `X-Repository` headers to every HTTP Response.
pub fn source_headers() -> DefaultHeaders {
    DefaultHeaders::new()
    .add(("Server", meta::USERAGENT))
    .add(("X-Authors", meta::AUTHORS.replace(':', ", ")))
    .add(("X-Repository", meta::REPOSITORY))
}