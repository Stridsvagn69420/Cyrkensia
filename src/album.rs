use serde::{Serialize, Deserialize};
use std::cmp::PartialEq;
use std::convert::From;
use super::{Artist, Metadata, add_vec, remove_vec};

/// Album
/// 
/// A struct representing an album of the Cyrkensia repository.
#[derive(Debug, Serialize, Deserialize)]
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
    /// The Artists of the album.
    pub artists: Vec<Artist>,

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
    pub fn new(name: String, cover: String, path: String, artists: Option<Vec<Artist>>, files: Option<Vec<String>>, size: u128) -> Album {
        Album {
            name,
            cover,
            path,
            artists: match artists {
                Some(x) => x,
                None => Vec::new()
            },
            files: match files {
                Some(x) => x,
                None => Vec::new()
            },
            size
        }
    }

    /// Add [Artist]
    /// 
    /// Adds an [Artist] to the album if they don't exist already.
    pub fn add_artist(&mut self, art: Artist) -> &mut Album {
        add_vec(&mut self.artists, art);
        self
    }

    /// Add file
    /// 
    /// Adds a file to the album if it doesn't exist already.
    pub fn add_file(&mut self, file: String) -> &mut Album {
        add_vec(&mut self.files, file);
        self
    }

    /// Remove [Artist]
    /// 
    /// Removes an [Artist] from the album if they already exist.
    pub fn remove_artist(&mut self, art: Artist) -> &mut Album {
        remove_vec(&mut self.artists, art);
        self
    }

    /// Remove file
    /// 
    /// Removes a file from the album if it already exists.
    pub fn remove_file(&mut self, file: String) -> &mut Album {
        remove_vec(&mut self.files, file);
        self
    }
}

impl From<Metadata> for Album {
    fn from(x: Metadata) -> Album {
        Album {
            name: x.name,
            cover: x.cover,
            path: "".to_string(),
            artists: x.artists,
            files: Vec::new(),
            size: 0
        }
    }
}

impl PartialEq for Album {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.cover == other.cover &&
        self.path == other.path &&
        self.artists == other.artists &&
        self.files == other.files &&
        self.size == other.size
    }
}