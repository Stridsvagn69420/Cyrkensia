use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::convert::From;
use super::Metadata;

/// Album
/// 
/// A struct representing an album of the Cyrkensia repository.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Album {
	/// Name
	/// 
	/// The name of the album.
	pub name: String,

	/// Cover
	/// 
	/// The album cover asset key.
	pub cover: String,

	/// Path
	/// 
	/// The relative path of the album.
	pub path: String,

	/// Files
	/// 
	/// All files present in the album as a relative path.
	pub files: HashMap<String, Vec<Uuid>>,

	/// Size
	/// 
	/// The size of the Album.
	pub size: u128
}

impl Album {
	/// New Album
	/// 
	/// Creates a new album. If `artists` or `files` is [None], an empty array will be created for them.
	pub fn new(name: String, cover: String, path: String, files: Option<HashMap<String, Vec<Uuid>>>, size: u128) -> Album {
		Album {
			name,
			cover,
			path,
			files: match files {
				Some(x) => x,
				None => HashMap::new()
			},
			size
		}
	}
}

impl From<Metadata> for Album {
	fn from(x: Metadata) -> Album {
		Album {
			name: x.name,
			cover: x.cover,
			path: "".to_string(),
			files: x.artists,
			size: 0
		}
	}
}

impl PartialEq for Album {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name &&
		self.cover == other.cover &&
		self.path == other.path &&
		self.files == other.files &&
		self.size == other.size
	}
}