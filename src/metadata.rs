use serde::{Serialize, Deserialize};
use serde_json::from_str;
use uuid::Uuid;
use std::collections::HashMap;
use std::fmt::{Display, Result};
use std::cmp::PartialEq;
use std::convert::From;
use std::path::Path;
use std::fs;
use std::io;
use super::Album;

/// .metdata.json
/// 
/// A struct representing an album's `.metadata.json` file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
	/// Name
	/// 
	/// The name of the album.
	pub name: String,

	/// Cover
	/// 
	/// The asset key representing the album cover art.
	pub cover: String,

	/// Default Artist
	/// 
	/// Represents the UUIDv4 of the main [Artist] of this album
	pub default: Option<Uuid>,

	/// Additional Artists
	/// 
	/// Map of which additional artists are associated with what music track
	pub artists: HashMap<String, Vec<Uuid>>
}

impl Metadata {
	/// New Metadata
	/// 
	/// Creates new [Metadata]. Authors can be appended later on.
	pub fn new(name: String, cover: String, default_artist: Option<Uuid>, artists: Option<HashMap<String, Vec<Uuid>>>) -> Metadata {
		Metadata {
			name,
			cover,
			default: default_artist,
			artists: match artists {
				Some(x) => x,
				None => HashMap::new()
			}
		}
	}

	/// Load .metadata.json
	/// 
	/// Loads a `.metadata.json` file into a [Metadata] instance.
	pub fn load(path: impl AsRef<Path>) -> io::Result<Metadata> {
		let data = fs::read_to_string(path)?;
		Ok(from_str(data.as_str())?)
	}
}

impl From<Album> for Metadata {
	fn from(x: Album) -> Metadata {
		Metadata {
			name: x.name,
			cover: x.cover,
			artists: x.files,
			default: None
		}
	}
}

impl PartialEq for Metadata {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name &&
		self.cover == other.cover &&
		self.artists == other.artists
	}
}

impl Display for Metadata {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
		write!(f, "{}", self.name)
	}
}