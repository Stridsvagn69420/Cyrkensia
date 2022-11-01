use actix_web::{HttpResponse, Responder, http::header::LOCATION};
use crate::meta;

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
pub async fn repository() -> impl Responder {
    redirect(meta::REPOSITORY)
}

/// License redirect
/// 
/// Redirects to the license text.
pub async fn license() -> impl Responder {
    redirect(meta::LICENSE_URL)
}