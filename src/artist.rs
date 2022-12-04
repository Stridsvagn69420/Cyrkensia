use serde::{Serialize, Deserialize};
use uuid::Uuid;
use dirs::home_dir;
use std::fmt::{Display, Result};
use std::cmp::PartialEq;
use std::convert::From;
use std::path::Path;
use std::{fs, io, env};
use super::{Owner, meta};

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

	/// Load artists.json
	/// 
	/// Loads a `artists.json` as a [Vec] of [Artist]s
	pub fn load(path: impl AsRef<Path>) -> io::Result<Vec<Artist>> {
		let rawdata = fs::read_to_string(path)?;
		Ok(serde_json::from_str(rawdata.as_str())?)
	}

	/// Cascade Loading
	/// 
	/// Attempts to load a config file by:
	/// 1. Provided [String]
	/// 2. `CYRKENSIA_ARTISTS` environment variable
	/// 3. `~/.config/cyrkensia/artists.json` file
	pub fn load_cascade(cmdarg: &Option<String>) -> io::Result<Vec<Artist>> {
		// Select extra path
		let envvar = env::var(meta::ARTISTS_ENVVAR);

		// Read config from extra location
		if let Some(path) = cmdarg.as_ref().or_else(|| envvar.as_ref().ok()) {
			return Artist::load(path);
		}

		// Read with default path
		let localpath = home_dir().unwrap_or_default().join(meta::ARTISTS_PATH);
		Artist::load(localpath)
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