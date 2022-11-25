use actix_web::middleware::DefaultHeaders;
use actix_web::http::header;
use actix_cors::Cors;
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

/// CORS everywhere middleware
/// 
/// Create a CORS Middleware that allows any origin.
pub fn cors_everywhere() -> Cors {
	// Cors Middleware
	Cors::default()
	.supports_credentials()
	.allow_any_origin()
	.max_age(86400)

	// --- Methods ---
	.allowed_methods(vec![
		"GET", "HEAD", "OPTIONS"
	])

	// --- Headers ---
	.allowed_headers(vec![
		header::ETAG,
		header::CACHE_CONTROL,
		header::IF_MODIFIED_SINCE,
		header::AUTHORIZATION,
		header::ACCEPT,
		header::ACCEPT_LANGUAGE,
		header::ACCEPT_ENCODING,
		header::CONTENT_TYPE,
		header::CONTENT_LENGTH,
		header::ORIGIN,
		header::DNT,
		header::USER_AGENT
	])
	.allowed_header("X-Requested-With")

	// --- Exposed ---
	.expose_headers(vec![
		header::WWW_AUTHENTICATE,
		header::CONTENT_LENGTH,
		header::CONTENT_ENCODING,
		header::CONTENT_RANGE,
		header::CONTENT_TYPE
	])
}