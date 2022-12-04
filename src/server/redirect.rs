use actix_web::{HttpResponse, Responder, http::header::LOCATION, HttpRequest};
use crate::meta;
use super::uri_noquery;

/// Redirect
/// 
/// Creates a new 301 Moved Permanently redirect with the given Url.
pub fn redirect(url: &str) -> impl Responder {
	HttpResponse::MovedPermanently()
	.append_header((LOCATION, url))
	.finish()
}

/// Repository redirect
/// 
/// Redirects to the source code repository.
pub fn repository() -> impl Responder {
	redirect(meta::REPOSITORY)
}

/// License redirect
/// 
/// Redirects to the license text.
pub fn license() -> impl Responder {
	redirect(meta::LICENSE_URL)
}

/// Trailing slash redirect
/// 
/// Redirects the user to a URL with a trailing slash.
pub async fn trail_slash(req: HttpRequest) -> impl Responder {
	let uri = uri_noquery(req.uri());
	HttpResponse::Found()
	.insert_header(("Location", uri + "/"))
	.finish()
}