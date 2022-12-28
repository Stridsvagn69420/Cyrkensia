use std::path::PathBuf;
use std::time::Instant;
use std::{io, fs};
use actix_web::{web, Responder, HttpResponse};
use actix_web::http::header::ContentType;
use serde::Deserialize;
use super::{CyrkensiaState, responses};
use crate::{Hostinfo, Artist, Metadata, Album};

/// Hostinfo Route
/// 
/// Route for serving a [Hostinfo]. Server needs [CyrkensiaState] in `.app_data()` for this.
pub async fn hostinfo(data: web::Data<CyrkensiaState>) -> impl Responder {
	// Get config
	let Some(delay) = data.config.max_age else {
		// Ad hoch Hostinfo
		let Ok(artists) = Artist::load_cascade(&data.config.artists) else {
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
		let Ok(new_artists) = Artist::load_cascade(&data.config.artists) else {
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
	let final_hostinfo = hostinfo.clone();

	// Return final result
	let Ok(finalres) = responses::hostinfo_data(&final_hostinfo) else {
		return responses::server_500(Some("Failed to generate hostinfo"));
	};
	finalres
}






/// Index Route Params
/// 
/// Simple struct containing the param-name and param-type needed for the [index] route.
#[derive(Deserialize)]
pub struct IndexParams {
	pub album: String
}

/// Album Index Route
/// 
/// Route for listing all files of a specific. Redundant, can be ignored.
pub async fn index(p: web::Path<IndexParams>, data: web::Data<CyrkensiaState>) -> impl Responder {
	// Extract album name
	let path = p.into_inner();

	// Read files and album name depending on if cache is enabled or not
	let meta: (String, usize, Vec<String>) = if data.config.max_age.is_some() {
		// Cached files and metadata
		let Ok(hostinfo) = data.hostinfo.lock() else {
			return responses::server_500(None::<String>);
		};
		let Some(album) = hostinfo.albums.iter().find(|x| x.path == path.album) else {
			return responses::client_404(Some("Album not found"));
		};

		// Return tri-tuple
		(album.name.clone(), album.files.len(), album.files.clone())
	} else {
		// Attempt to find album in filesystem
		let Ok(path) = Album::find(&data.config.root, &path.album) else {
			return responses::client_404(Some("Album not found"));
		};

		// Ad-hoc metadata and files
		let Ok(album) = Metadata::load(path.join(".metadata.json")) else {
			return responses::client_404(Some("Album not found"));
		};
		let Ok(files) = Hostinfo::list_files(path) else {
			return responses::client_404(Some("Album not accessable"));
		};

		// Return tri-tuple
		(album.name, files.0.len(), files.0)
	};

	// Codegen
	let headmeta = r###"<meta name="viewport" content="width=device-width, initial-scale=1.0">
	<style>
	h2 { color: white; text-decoration: underline; }
	a { color: cyan; margin: 8px; }
	body { font-family: sans-serif, system-ui; background-color: #252545; }
	</style>
	"###;
	let headstr = format!("<h2>{} ({})</h2>", meta.0, meta.1);
	let bodystr = meta.2.into_iter().fold(String::new(), |total, item| total + &format!("<a href=\"{}\">{}</a><br>\n", item, item));

	// Send response
	HttpResponse::Ok()
	.content_type(ContentType::html())
	.body(format!("<html><head>{}</head><body>{}{}</body></html>", headmeta, headstr, bodystr))
}




/// File Route Params
/// 
/// Simple struct for the path parameters used in [file_head] and [file_serving]
#[derive(Deserialize)]
pub struct FileParams {
	pub albun: String,
	pub file: String
}

impl FileParams {
	/// Find file path
	/// 
	/// Attempts to find the file path in the filesystem.
	pub fn find_file(roots: &[String], album: &String, file: String) -> io::Result<PathBuf> {
		let album_path = Album::find(roots, album)?;
		// Read found directory
		let filepath = fs::read_dir(album_path)?.into_iter()
		.filter_map(|x| {
			let Ok(dentry) = x else {
				return None;
			};
			Some(dentry)
		})
		// Convert filename
		.filter_map(|y| {
			if let Some(fname) = y.file_name().to_str() {
				return Some((fname.to_string(), y.path()));
			}
			None
			
		})
		// Find file
		.find(|z| z.0 == file);

		// Extract path
		if let Some(stuff) = filepath {
			return Ok(stuff.1);
		}
		Err(io::Error::new(io::ErrorKind::NotFound, "Could not find file"))
	}
}

/// File Head Route
/// 
/// 
pub async fn file_serving(p: web::Path<FileParams>, data: web::Data<CyrkensiaState>) -> impl Responder {
	let path = p.into_inner();

	// Generate ETag with BLAKE3 hash as Hexadecimal
	// Generate Digest header with BLAKE3, SHA-256, SHA-512 as Base64
	// Generate Last-Modified header from fs::metadata
	// Match HTTP Status Codes

	HttpResponse::Ok()
}

/// File Serving Route
/// 
/// 
pub async fn file_head(p: web::Path<FileParams>, data: web::Data<CyrkensiaState>) -> impl Responder {
	let pathdata = p.into_inner();
	let Ok(path) = FileParams::find_file(&data.config.root, &pathdata.albun, pathdata.file) else {
		return responses::client_404(Some("File not found"));
	};

	// Generate ETag with BLAKE3 hash as Hexadecimal
	// Generate Digest header with BLAKE3, SHA-256, SHA-512 as Base64
	// Generate Last-Modified header from fs::metadata
	// Include If-Modified-Since behaviour and match HTTP Status Codes

	HttpResponse::Ok().finish()
}