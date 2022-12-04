use serde::{Serialize, Deserialize};
use std::cmp::PartialEq;
use std::convert::From;
use std::ffi::OsString;
use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use super::{Owner, Album, Config, Metadata, Artist};

/// Hostinfo
/// 
/// A struct representing the metadata or hostinfo and index of a Cyrkensia repository.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hostinfo {
	/// Name
	/// 
	/// The name of the Cyrkensia repository.
	pub name: String,

	/// Icon
	/// 
	/// The icon representing the Cyrkensia repo.
	pub icon: String,

	/// UUID
	/// 
	/// The UUIDv4 of the repository.
	pub uuid: Uuid,

	/// Secured
	/// 
	/// Determines whether a repository requires HTTP Basic Auth or not.
	pub secured: bool,

	/// Size in bytes
	/// 
	/// The total size of the repository and its music files in bytes.
	pub size: u128,

	/// Origin URL
	/// 
	/// The URL to get an updated version of this Hostinfo
	pub origin: String,

	/// Albums
	/// 
	/// List of all albums that this Cyrkensia repository provides.
	pub albums: Vec<Album>,

	/// Maintainers and Owners
	/// 
	/// List of all repository maintainers/owners.
	pub owners: Vec<Owner>,

	/// Featured Artists
	/// 
	/// List of all [Artist]s in this repository.
	pub artists: Vec<Artist>
}

impl Hostinfo {
	/// Set Origin
	/// 
	/// Sets the origin of the Hostinfo. This function solely exists to remind library users to swap out the empty default origin.
	pub fn set_origin(&mut self, org: String) {
		self.origin = org
	}

	/// Generate empty Hostinfo
	/// 
	/// Generates an empty Hostinfo.
	pub fn empty() -> Hostinfo {
		Hostinfo {
			name: "".to_string(),
			icon: "".to_string(),
			uuid: Uuid::new_v4(),
			secured: false,
			size: 0,
			origin: "".to_string(),
			albums: Vec::new(),
			owners: Vec::new(),
			artists: Vec::new()
		}
	}

	/// Generate Hostinfo
	/// 
	/// Generates a Hostinfo based on a [Config].
	pub fn generate(cfg: &Config, arts: &Vec<Artist>) -> io::Result<Hostinfo> {
		let mut albums: Vec<Album> = Vec::new();
		for rootpath in &cfg.root {
			let album_slice = Hostinfo::read_albums(rootpath)?;
			albums.extend(album_slice);
		}
		let mut hostinfo = Hostinfo::from(cfg.clone());
		hostinfo.size = albums.iter().map(|x| x.size).sum();
		hostinfo.albums = albums;
		hostinfo.artists = arts.to_owned();
		Ok(hostinfo)
	}

	/// Read Albums
	/// 
	/// Reads a given directory and returns it as a [Vec] of [Album]s.
	pub fn read_albums(path: impl AsRef<Path>) -> io::Result<Vec<Album>> {
		// Read given Cyrkensia root
		let albums: Vec<Album> = fs::read_dir(path.as_ref())?

		// Only use working entries and get directory path and name
		.filter_map(|x| {
			// Try getting the DirEntry
			let Ok(dir) = x else {
				return None;
			};
			// Filter out non-dirs
			if !dir.path().is_dir() {
				return None;
			}
			// Return path and filename
			Some((dir.path(), dir.file_name()))
		})

		// Create Album instances
		.filter_map(|y| {
			Hostinfo::parse_album(&y.0, &y.1).ok()
		})

		// Collect and Return
		.collect();
		Ok(albums)
	}

	/// Parse album
	/// 
	/// This is a wrapper for the second `.filter_map()` in [read_albums](Hostinfo::read_albums) since clojures don't allow the `?`.
	/// You can of course also use it if you desire.
	pub fn parse_album(path: &PathBuf, name: &OsString) -> io::Result<Album> {
		let Some(fname) = name.to_str() else {
			// Return NotFound error
			return Err(io::Error::new(io::ErrorKind::NotFound, "The OsString could not be parsed to a String"));
		};

		// Read data
		let m = Metadata::load(path.join(".metadata.json"))?;
		let c = Hostinfo::list_files(path)?;

		// Create Album
		Ok(Album {
			name: m.name,
			cover: m.cover,
			path: fname.to_string(),
			files: c.0,
			size: c.1,
    		artists: m.artists,
		})
	}

	/// List files
	/// 
	/// Lists all files in a given directory and their total size
	pub fn list_files(path: impl AsRef<Path>) -> io::Result<(Vec<String>, u128)> {
		let mut allsize: u128 = 0;

		// Read Dir
		let file_entries = fs::read_dir(path)?

		// Only files and with successful metadata
		.filter_map(|x| {
			// Get successful DirEntries
			let Ok(entry) = x else {
				return None;
			};
			// Filter out non-files
			if !entry.path().is_file() {
				return None;                
			}
			// Get successful Metadata
			let Ok(fmeta) = entry.metadata() else {
				return None;
			};
			// Return Path and Length
			Some((entry.path(), fmeta.len()))
		})
		
		// Return only the filename and not the entire path
		.filter_map(|y| {
			// Get OS filename
			let Some(filename) = y.0.file_name() else {
				return None;
			};
			// Get filename as &str
			let Some(fname) = filename.to_str() else {  
				return  None;
			};
			// Filter out dotfiles  
			if fname.starts_with('.') {
				return None;
			}
			// Append to size and return filename
			allsize += y.1 as u128;
			Some(fname.to_string())
		})

		// Collect to Vec<String> and return result
		.collect();
		Ok((file_entries, allsize))
	}
}

impl From<Config> for Hostinfo {
	fn from(x: Config) -> Hostinfo {
		Hostinfo {
			name: x.name,
			icon: x.icon,
			uuid: x.uuid,
			secured: x.htpasswd.is_some(),
			size: 0,
			origin: "".to_string(),
			albums: Vec::new(),
			owners: x.owners,
			artists: Vec::new()
		}
	}
}

impl PartialEq for Hostinfo {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name &&
		self.icon == other.icon &&
		self.uuid == other.uuid &&
		self.secured == other.secured &&
		self.size == other.size &&
		self.origin == other.origin &&
		self.albums == other.albums &&
		self.owners == other.owners
	}
}