use serde::{Serialize, Deserialize};

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

// TODO: Add trait impls here and other useful functions