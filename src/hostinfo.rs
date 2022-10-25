use serde::{Serialize, Deserialize};
use super::{Owner, Album};

/// Hostinfo
/// 
/// A struct representing the metadata or hostinfo and index of a Cyrkensia repository.
#[derive(Debug, Serialize, Deserialize)]
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
    pub uuid: String,

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
    pub owners: Vec<Owner>
}

// TODO: Add trait impls and useful functions here