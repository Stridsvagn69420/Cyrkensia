use std::time::Instant;
use std::path::Path;
use actix_web::{web, Responder, HttpRequest, HttpResponse};
use actix_web::http::header::ContentType;
use super::{CyrkensiaState, responses, uri_noquery};
use crate::{Hostinfo, Artist, Metadata};

/// Hostinfo Route
/// 
/// Route for serving a [Hostinfo]. Server needs [CyrkensiaState] in `.app_data()` for this.
pub async fn hostinfo(req: HttpRequest, data: web::Data<CyrkensiaState>) -> impl Responder {
	// Get config
	let Some(delay) = data.config.max_age else {
		// Ad hoch Hostinfo
		let Ok(artists) = Artist::read_multiple(&data.config.root) else {
			return responses::server_500(Some("Failed to generate hostinfo"));
		};
		let Ok(resp) = responses::hostinfo_json(&data.config, &artists) else {
			return responses::server_500(Some("Failed to generate hostinfo"));
		};
		return resp;
	};

	// Get last update timestamp and cached hostinfo and artists
	let Ok(mut last_updated) = data.last_updated.lock() else {
		return responses::server_500(None::<String>);
	};
	let Ok(mut hostinfo) = data.hostinfo.lock() else {
		return responses::server_500(None::<String>);
	};
	let Ok(mut artists) = data.artists.lock() else {
		return responses::server_500(None::<String>);
	};

	// Update Cache if expired
	if last_updated.elapsed().as_secs() >= delay {
		// Read updated artists
		let Ok(new_artists) = Artist::read_multiple(&data.config.root) else {
			return responses::server_500(Some("Failed to update hostinfo"));
		};

		// Generate new Hostinfo
		let Ok(new_hostinfo) = Hostinfo::generate(&data.config, &artists) else {
			return responses::server_500(Some("Failed to update hostinfo"));
		};

		// Update Hostinfo and Timestamp
		*artists = new_artists;
		*hostinfo = new_hostinfo;
		*last_updated = Instant::now();
	}

	// Set Origin URL
	let mut final_hostinfo = hostinfo.clone();
	final_hostinfo.set_origin(uri_noquery(req.uri()));

	// Return final result
	let Ok(finalres) = responses::hostinfo_data(&final_hostinfo) else {
		return responses::server_500(Some("Failed to generate hostinfo"));
	};
	finalres
}

/// Index Route Params
/// 
/// Simple struct containing the param-name and param-type needed for the [index] route.
pub struct IndexParams {
	album: String
}

/// Album Index Route
/// 
/// Route for listing all files of a specific. Redundant, can be ignored.
pub async fn index(p: web::Path<IndexParams>, data: web::Data<CyrkensiaState>) -> impl Responder {
	// Read files and album name depending on if cache is enabled or not
	let meta: (String, usize, Vec<String>) = match data.config.max_age {
		Some(_) => {
			// Cached files and metadata
			let Ok(hostinfo) = data.hostinfo.lock() else {
				return responses::server_500(None::<String>);
			};
			let Some(album) = hostinfo.albums.clone().into_iter().find(|x| x.path == p.album) else {
				return responses::client_404(Some("Album not found"));
			};

			// Return tri-tuple
			let files: Vec<String> = album.files.keys().cloned().collect();
			(album.name, files.len(), files)

		},
		None => {
			// Ad-hoc files and metadata
			let Ok(files) = Hostinfo::list_files(&p.album) else {
				return responses::client_404(Some("Album not found or accessable"));
			};
			let Ok(album) = Metadata::load(Path::new(p.album.as_str()).join(".metadata.json")) else {
				return responses::client_404(Some("Album not found"));
			};

			// Return tri-tuple
			(album.name, files.0.len(), files.0)
		}
	};

	// Codegen
	let headstr = format!("<h3>{} ({})</h3>", meta.0, meta.1);
	let bodystr: String = meta.2.into_iter().fold(String::new(), |total, item| total + &format!("<a href=\"{}\">{}</a><br>\n", item, item));

	// Send response
	HttpResponse::Ok()
	.content_type(ContentType::html())
	.body(headstr + &bodystr)
}