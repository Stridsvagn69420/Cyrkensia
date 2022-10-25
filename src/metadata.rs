use serde::{Serialize, Deserialize};
use super::Artist;

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

    /// Authors
    /// 
    /// List of Authors in this album.
    pub authors: Vec<Artist>
}

// TODO: Add trait impls and other functions