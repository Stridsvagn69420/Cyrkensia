use std::io;
use actix_web::body::MessageBody;
use actix_web::{HttpResponse, HttpResponseBuilder};
use actix_web::http::header::{ContentType, CONTENT_LENGTH, WWW_AUTHENTICATE};
use crate::{Config, Hostinfo, Artist};

/// Hostinfo from Config
/// 
/// Attempts to create a [HttpResponse] from a generated [Hostinfo] by a [Config].
pub fn hostinfo_json(cfg: &Config, arts: &Vec<Artist>) -> io::Result<HttpResponse> {
	// Generate Hostinfo
	let hostinfo = Hostinfo::generate(cfg, arts)?;
	hostinfo_data(&hostinfo)
}

/// Hostinfo from Data
/// 
/// Attempts to create a [HttpResponse] from a [Hostinfo] struct.
pub fn hostinfo_data(hstinfo: &Hostinfo) -> io::Result<HttpResponse> {
	// Convert to String
	let raw_json = serde_json::to_string(hstinfo)?;
	
	// Return HttpReponse
	Ok(HttpResponse::Ok()
	.content_type(ContentType::json())
	.append_header(
		(CONTENT_LENGTH, raw_json.len())
	)
	.body(raw_json))
}

/// HTTP Status 500
/// 
/// Returns a 500 Error with an optional body message
pub fn server_500(msg: Option<impl MessageBody + 'static>) -> HttpResponse {
	error(HttpResponse::InternalServerError(), msg)
}

/// HTTP Status 404
/// 
/// Returns a 404 Error with an optional body message
pub fn client_404(msg: Option<impl MessageBody + 'static>) -> HttpResponse {
	error(HttpResponse::NotFound(), msg)
}

/// HTTP Status 401
/// 
/// Returns a 401 Error with an optional body message
pub fn client_401(msg: Option<impl MessageBody + 'static>) -> HttpResponse {
	let mut resp = HttpResponse::Unauthorized();
	resp.insert_header((WWW_AUTHENTICATE, "Basic realm=\"Cyrkensia\""));
	error(resp, msg)
}

/// General Purpose Status Reponse Builder
/// 
/// Just makes the [HttpResponseBuilder] as a finished [HttpResponse], optionally being able to pass a body.
pub fn error(mut resp: HttpResponseBuilder, msg: Option<impl MessageBody + 'static>) -> HttpResponse {
	if let Some(message) = msg {
		return resp.body(message)
	}
	resp.finish()
}