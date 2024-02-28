use actix_web::http::header::HeaderValue;
use base64::encode;
use blake3::{hash, Hash};
use chrono::{DateTime, Utc};
use sha2::{Sha256, Sha512, Digest};
use actix_web::http::Uri;
use crate::account::Account;
use crate::{Hostinfo, Config, Artist};
use std::ffi::OsStr;
use std::fmt::Write;
use std::sync::Mutex;
use std::time::Instant;
use std::{io, fs};
use std::path::Path;

/// Middlewares
/// 
/// Submodule containing Cyrkensia middlewares.
pub mod middleware;

/// Redirects
/// 
/// Submodule containing Cyrkensia redirects.
pub mod redirect;

/// Routes
/// 
/// Submodule containing Cyrkensia routes.
pub mod routes;

/// Responses
/// 
/// Submodue containing Cyrkensia responses.
pub mod responses;

/// Cyrkensia State
/// 
/// State for the Actix-Web server. Used in [routes].
pub struct CyrkensiaState {
	/// Config
	/// 
	/// The loaded Config (read-only)
	pub config: Config,

	/// Accounts
	/// 
	/// The optionally loaded Account database (read-only)
	pub accounts: Vec<Account>,

	/// Artists
	/// 
	/// The lateast loaded Artists
	pub artists: Mutex<Vec<Artist>>,

	/// Hostinfo
	/// 
	/// The latest generated [Hostinfo].
	/// Only used if caching is activated.
	pub hostinfo: Mutex<Hostinfo>,

	/// Last Hostinfo Update
	/// 
	/// The [timestamp](Instant) when the [Hostinfo] was last updated.
	/// `.elapsed().as_secs()` will be used to compare it with the `max_age` in the [Config].
	/// Only used if caching is activated.
	pub last_updated: Mutex<Instant>
}

/// Uri Display without Query
/// 
/// Displays a Uri without the query parameters
pub fn uri_noquery(uri: &Uri) -> String {
	let mut f = String::new();

	// Protocol
	if let Some(scheme) = uri.scheme() {
		let _ = write!(&mut f, "{}://", scheme);
	}
	// Server
	if let Some(authority) = uri.authority() {
		let _ = write!(&mut f, "{}", authority);
	}
	// Path
	let _ = write!(&mut f, "{}", uri.path());

	f
}

impl CyrkensiaState {
	/// Constructur
	/// 
	/// Creates a new [CyrkensiaState] with given [Config].
	pub fn new(cfg: Config) -> io::Result<CyrkensiaState> {
		let accounts = match &cfg.htpasswd {
			Some(path) => Account::load(path)?,
			None => Vec::new()
		};

		// State with caching
		if cfg.max_age.is_some() {
			let arts = Artist::load_cascade(&cfg.artists)?;
			let hostinfo = Hostinfo::generate(&cfg, &arts)?;
			return Ok(CyrkensiaState {
				last_updated: Mutex::new(Instant::now()),
				hostinfo: Mutex::new(hostinfo),
				artists: Mutex::new(arts),
				config: cfg,
				accounts
			});
		}

		// State without caching
		Ok(CyrkensiaState {
			hostinfo: Mutex::new(Hostinfo::empty()),
			last_updated: Mutex::new(Instant::now()),
			artists: Mutex::new(Vec::new()),
			config: cfg,
			accounts
		})
	}
}

/// Timestamp
/// 
/// Creates the current timestamp formatted as `[%Y-%m-%d %H:%M:%S]`.
/// Requires [chrono::Local].
/// Evaluates to `Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string()`.
#[macro_export]
macro_rules! timestamp {
	() => {
		Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string()
	};
}
pub use timestamp;

/// Timelog
/// 
/// Formats a given message with a timestamp.
/// If no message is provided, it just returns [timestamp!], but with a whitespace added.
#[macro_export]
macro_rules! timelog {
	() => {
		(timestamp!() + " ").as_str()
	};
	($msg:expr) => {
		(timestamp!() + " " + $msg).as_str()
	};
}
pub use timelog;

/// Hashgen
/// 
/// Generates the Blake3, SHA-512 and SHA-256 hashes in Base64.
pub fn hashgen(data: &[u8]) -> (Hash, String, String) {
	// Generate hash
	let sha256hash = Sha256::digest(data);
	let sha512hash = Sha512::digest(data);
	let b3hash = hash(data);

	// Return tuple
	(b3hash, encode(sha512hash), encode(sha256hash))
}

/// Filetime
/// 
/// Attempts to read a file's last modified date and returns it as an RFC 2822 string with the offset replaced with GMT.
/// Returns a tuple of the formatted timestamp as a [String] and the time as [DateTime] with [Utc] timezone.
pub fn filetime(p: impl AsRef<Path>) -> io::Result<(String, DateTime<Utc>)> {
	let systime = fs::metadata(p)?.modified()?;
	let datetime: DateTime<Utc> = systime.into();
	Ok((datetime.to_rfc2822().replace("+0000", "GMT"), datetime))
}

/// Compare time
/// 
/// Attempts to parse the If-Modified-Since timestamp and compare it to the given [filetime] [result](DateTime).
/// Returns true if the If-Modified-Since condition is met, otherwise false.
pub fn compare_time(ftime: DateTime<Utc>, ims_head: &HeaderValue) -> io::Result<bool> {
	let Ok(x) = ims_head.to_str().map(|x| x.to_string()) else {
		return Err(io::Error::new(io::ErrorKind::NotFound, "Could not extract header value"));
	};
	let Ok(rtime) = DateTime::parse_from_rfc2822(x.as_str()) else {
		return Err(io::Error::new(io::ErrorKind::NotFound, "Could not parse time"));
	};
	Ok(ftime > rtime)
}

/// Get MIME-Type
/// 
/// Gets the MIME-Type for a given file extension.
pub fn get_mime(extop: Option<&OsStr>) -> &str {
	match extop.and_then(|x| x.to_str()) {
		Some("aac") => "audio/aac",
		Some("mp3") => "audio/mp3",
		Some("opus") => "audio/opus",
		Some("m4a") => "audio/mp4",
		Some("wav") => "audio/wav",
		Some("3gp") => "audio/3gpp",
		Some("3g2") => "audio/3gpp2",
		Some("mid") | Some("midi") => "audio/midi",
		Some("oga") | Some("ogg") => "audio/ogg",
		Some("weba") | Some("webm") => "audio/webm",
		_ => "application/octet-stream"
	}
}