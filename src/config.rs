use serde::{Serialize, Deserialize};
use serde_json::from_str;
use dirs::home_dir;
use uuid::Uuid;
use std::cmp::PartialEq;
use std::convert::From;
use std::path::Path;
use std::io;
use std::fs;
use std::env;
use super::{Owner, Hostinfo, meta};

/// Configuration
/// 
/// The server configuration used for Cyrkensia.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
	/// Name
	/// 
	/// The display name of the repository.
	pub name: String,

	/// File roots
	/// 
	/// The file roots where all albums are stored. 
	pub root: Vec<String>,

	/// UUIDv4
	/// 
	/// The UUID of the repository.
	pub uuid: Uuid,

	/// Icon
	/// 
	/// The Rich Presence asset key used for displaying the server.
	pub icon: String,

	/// Account file
	/// 
	/// The path to the Account file for HTTP-Basic Authentication. If none is present, HTTP Basic Auth will be disabled.
	/// Note that the file must be a JSON of [Vec]<[Account](super::account::Account)>.
	pub htpasswd: Option<String>,

	/// Bind address
	/// 
	/// The IP address to bind to, e.g. `127.0.0.1`, `0.0.0.0:80` or a Unix socket (Unix only).
	pub bindaddr: String,

	/// Owners
	/// 
	/// List of repository [maintainers](Owner)
	pub owners: Vec<Owner>,

	/// Maximum Age
	/// 
	/// The maximum age of the [Hostinfo] in milliseconds as a [u64]. If [None], the Hostinfo will always be regenerated when its route is accessed.
	/// This basically activates caching.
	pub max_age: Option<u64>
}

impl Config {
	/// Load Config File
	/// 
	/// Loads a config from a file in the filesystem.
	pub fn load_file(path: impl AsRef<Path>) -> io::Result<Config> {
		let rawfile = fs::read_to_string(path)?;
		Ok(from_str(rawfile.as_str())?)
	}

	/// Load Config JSON
	/// 
	/// Loads a config from an already existing &[str].
	pub fn load_json(data: &str) -> io::Result<Config> {
		Ok(from_str(data)?)
	}

	/// Cascade Loading
	/// 
	/// Attempts to load a config file by:
	/// 1. First command-line argument
	/// 2. `CYRKENSIA_CONFIG` environment variable
	/// 3. `~/.config/cyrkensia/config.json` file
	pub fn load_cascade(cmdarg: Option<&String>) -> io::Result<Config> {
		// Select extra path
		let envvar = env::var(meta::CONFIG_ENVVAR);

		// Read config from extra location
		if let Some(path) = cmdarg.or_else(|| envvar.as_ref().ok()) {
			return Config::load_file(path);
		}

		// Read with default path
		let localpath = home_dir().unwrap_or_default().join(meta::CONFIG_PATH);
		Config::load_file(localpath)
	}
}

impl From<Hostinfo> for Config {
	fn from(x: Hostinfo) -> Config {
		Config {
			name: x.name,
			root: Vec::new(),
			uuid: x.uuid,
			icon: x.icon,
			htpasswd: None,
			bindaddr: "".to_string(),
			owners: x.owners,
			max_age: None
		}
	}
}

impl PartialEq for Config {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name &&
		self.root == other.root &&
		self.uuid == other.uuid &&
		self.icon == other.icon &&
		self.htpasswd == other.htpasswd &&
		self.bindaddr == other.bindaddr &&
		self.owners == other.owners
	}
}