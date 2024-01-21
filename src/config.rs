use std::convert::TryFrom;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
	/// Server Name
	/// 
	/// The Name of the Cyrkensia server.
	pub name: String,

	/// Contact
	/// 
	/// A URL or E-Mail address to contact the server's maintainers.
	pub contact: String,

	/// Bind Address
	/// 
	/// The IP address and Port to bind to.
	/// On Unix platforms, this can also be a path to a Unix Domain Socket.
	pub addr: String,

	/// Music Database Path
	/// 
	/// Path to a folder that contains the `artists.json`, `album.json` and `tracks.json`.
	pub database: String,

	/// User Database Path
	/// 
	/// Path to a `users.json` file. If this is not set, the server will not ask for authorization.
	/// If it is set, but the file fails to be read, the server will not start.
	pub accounts: Option<String>
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