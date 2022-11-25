use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::fmt::{Display, Result};
use std::cmp::PartialEq;
use std::convert::From;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use super::Owner;

/// Artist
/// 
/// A struct representing an author or artist of a song.
/// An array of this is stored in a root of a Cyrkensia repository as a `.artists.json` file.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Artist {
	pub name: String,

	/// Website
	/// 
	/// The website of the artist.
	pub website: Option<String>,

	/// Uuid
	/// 
	/// The artist's UUID
	pub uuid: Uuid
}

impl Artist {
	/// New Artist
	/// 
	/// Creates a new Artist
	pub fn new(name: String, website: Option<String>, uuid: Uuid) -> Artist {
		Artist {
			name,
			website,
			uuid
		}
	}

	/// Load single .artists.json
	/// 
	/// Loads a single `.artists.json` as a [Vec] of [Artist]s
	pub fn load_artists(path: impl AsRef<Path>) -> io::Result<Vec<Artist>> {
		let rawdata = fs::read_to_string(path)?;
		Ok(serde_json::from_str(rawdata.as_str())?)
	}

	/// Load multiple .artists.json
	/// 
	/// Loads multiple `.artists.json` as a combined [Vec] of [Artist]s
	pub fn load_multiple_artists(paths: Vec<impl AsRef<Path>>) -> io::Result<Vec<Artist>> {
		let mut res: Vec<Artist> = Vec::new();
		for pth in paths {
			let artst = Artist::load_artists(pth)?;
			res.extend(artst);
		}
		Ok(res)
	}

	/// Read multiple folders' .artists.json
	/// 
	/// Reads the .artists.json of multiple folders. Essentially like [load_multiple_artists], but with `.artists.json` appended.
	pub fn read_multiple(paths: &[String]) -> io::Result<Vec<Artist>> {
		let conv_paths: Vec<PathBuf> = paths.iter()
		.map(|x| Path::new(x).join(".artists.json")).collect();
		
		// Read all artists
		Artist::load_multiple_artists(conv_paths)
	}
}

impl From<Owner> for Artist {
	fn from(x: Owner) -> Artist {
		Artist {
			name: x.name,
			website: x.website,
			uuid: Uuid::new_v4()
		}
	}
}

impl Display for Artist {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
		if let Some(web) = self.website.clone() {
			write!(f, "{} ({}) {}", self.name, web, self.uuid)
		} else {
			write!(f, "{} {}", self.name, self.uuid)
		}
	}
}

impl PartialEq for Artist {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name &&
		self.website == other.website &&
		self.uuid == other.uuid
	}
}