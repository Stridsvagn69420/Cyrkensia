use std::fmt;
use std::convert::TryFrom;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Config {
	/// Server Name
	/// 
	/// The Name of the Cyrkensia server.
	pub name: String,

	/// Bind Address
	/// 
	/// The IP address and Port to bind to.
	/// On Unix platforms, this can also be a path to a Unix Domain Socket.
	pub addr: String,

	/// Database Path
	/// 
	/// Path to the folder that contains the
	/// `artists.json`, `albums.json`, `tracks.json`, `users.json` and `maintainers.json` database files.
	pub database: String
}

impl TryFrom<&str> for Config {
	type Error = serde_json::Error;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		serde_json::from_str(value)
	}
}

impl TryFrom<String> for Config {
	type Error = serde_json::Error;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		serde_json::from_str(&value)
	}
}

impl TryFrom<&[u8]> for Config {
	type Error = serde_json::Error;

	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
		serde_json::from_slice(value)
	}
}

impl fmt::Display for Config {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}@{} ({})", self.name, self.addr, self.database)
	}
}