use serde::{Serialize, Deserialize};
use std::fmt::{Display, Result};
use std::cmp::PartialEq;
use std::convert::From;
use super::{Owner, add_vec, remove_vec};

/// Artist
/// 
/// A struct representing an author or artist of a song.
#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub name: String,

    /// Website
    /// 
    /// The website of the artist.
    pub website: Option<String>,

    /// Own music
    /// 
    /// List of files (music) that the artist made.
    pub music: Vec<String>,

    /// Features
    /// 
    /// List of files (music) that the artist was featured in.
    pub features: Vec<String>
}

impl Artist {
    /// New Artist
    /// 
    /// Creates a new Artist
    pub fn new(name: String, website: Option<String>, music: Vec<String>, features: Vec<String>) -> Artist {
        Artist {
            name,
            website,
            music,
            features
        }
    }

    /// Add music
    /// 
    /// Adds a new music entry if it doesn't exist already
    pub fn add_music(&mut self, music: String) -> &mut Artist {
        add_vec(&mut self.music, music);
        self
    }

    /// Add feature
    /// 
    /// Adds a new featured entry if it doesn't exist already
    pub fn add_feature(&mut self, featured: String) -> &mut Artist {
        add_vec(&mut self.features, featured);
        self
    }

    /// Remove music
    /// 
    /// Removes a music entry if it exists
    pub fn remove_music(&mut self, music: String) -> &mut Artist {
        remove_vec(&mut self.music, music);
        self
    }

    /// Remove feature
    /// 
    /// Removes a featured entry if it exists
    pub fn remove_feature(&mut self, featured: String) -> &mut Artist {
        remove_vec(&mut self.features, featured);
        self
    }
}

impl From<Owner> for Artist {
    fn from(x: Owner) -> Artist {
        Artist {
            name: x.name,
            website: x.website,
            music: Vec::new(),
            features: Vec::new()
        }
    }
}

impl Display for Artist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        if let Some(web) = self.website.clone() {
            write!(f, "{} ({})", self.name, web)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

impl PartialEq for Artist {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.website == other.website &&
        self.music == other.music &&
        self.features == other.features
    }
}