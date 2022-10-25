use serde::{Serialize, Deserialize};
use super::Artist;

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
    pub authors: Vec<Artist>,

    /// Files
    /// 
    /// All files present in the album as a relative path.
    pub files: Vec<String>
}

// TODO: Add trait impls and useful functions here