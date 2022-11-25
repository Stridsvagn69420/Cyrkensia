use actix_web::http::Uri;
use crate::account::Account;
use crate::{Hostinfo, Config, Artist};
use std::fmt::Write;
use std::sync::Mutex;
use std::time::Instant;
use std::io;

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
	pub accounts: Option<Vec<Account>>,

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
			Some(path) => Some(Account::load(path)?),
			None => None
		};

		// State with caching
		if cfg.max_age.is_some() {
			let arts = Artist::read_multiple(&cfg.root)?;
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