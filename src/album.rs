use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::fs;
use std::io;
use std::cmp::PartialEq;
use std::convert::From;
use std::path::PathBuf;
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

	/// Artists
	/// 
	/// The UUID of the artists responsible for this album.
	pub artists: Vec<Uuid>,

	/// Files
	/// 
	/// All files present in the album as a relative path.
	pub files: Vec<String>,

	/// Size
	/// 
	/// The size of the Album.
	pub size: u128
}

impl Album {
	/// New Album
	/// 
	/// Creates a new album. If `artists` or `files` is [None], an empty array will be created for them.
	pub fn new(name: String, cover: String, path: String, artists: Vec<Uuid>, files: Vec<String>, size: u128) -> Album {
		Album {
			name,
			cover,
			path,
			files,
			size,
    		artists
		}
	}

	/// Find Album in filesystem
	/// 
	/// Attempts to find an album in given roots by set folder name
	pub fn find(roots: &[String], name: &String) -> io::Result<PathBuf> {
		for root in roots {
			let item = fs::read_dir(root)?.into_iter()
			// Filter out invalid entries
			.filter_map(|x| x.ok())
			// Get dirname as String
			.filter_map(|x| {
				let path = x.path();
				if let Some(filename) = path.file_name()
				.and_then(|y| y.to_str()).map(|z| z.to_string()) {
        			return Some((filename, path));
    			}
				None
			})
			// Find album
			.find(|item| &item.0 == name);

			// Return first match
			if let Some(path) = item {
				return Ok(path.1);
			}
		}
		Err(io::Error::new(io::ErrorKind::NotFound, "Could not find album"))
	}
}

impl From<Metadata> for Album {
	fn from(x: Metadata) -> Album {
		Album {
			name: x.name,
			cover: x.cover,
			path: "".to_string(),
			files: Vec::new(),
			size: 0,
			artists: x.artists
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