use serde::{Serialize, Deserialize};
use std::fmt::{Display, Result};
use std::cmp::PartialEq;
use std::convert::From;
use super::{Artist, Album, add_vec, remove_vec};

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

    /// Artists
    /// 
    /// List of [Artist]s in this album.
    pub artists: Vec<Artist>
}

// TODO: Add trait impls and other functions

impl Metadata {
    /// New Metadata
    /// 
    /// Creates new [Metadata]. Authors can be appended later on.
    pub fn new(name: String, cover: String, artists: Option<Vec<Artist>>) -> Metadata {
        Metadata {
            name,
            cover,
            artists: match artists {
                Some(x) => x,
                None => Vec::new()
            }
        }
    }

    /// Add Artist
    /// 
    /// Adds an [Artist], if they don't exist yet.
    pub fn add_artist(&mut self, art: Artist) -> &mut Metadata {
        add_vec(&mut self.artists, art);
        self
    }

    /// Remove Artist
    /// 
    /// Removes an [Artist], if they already exist.
    pub fn remove_artist(&mut self, art: Artist) -> &mut Metadata {
        remove_vec(&mut self.artists, art);
        self
    }
}

impl From<Album> for Metadata {
    fn from(x: Album) -> Metadata {
        Metadata {
            name: x.name,
            cover: x.cover,
            artists: x.artists
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