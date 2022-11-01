use serde::{Serialize, Deserialize};
use std::cmp::PartialEq;
use std::convert::From;
use std::io;
use std::fs;
use std::path::Path;
use super::{Owner, Album, Config, Metadata};

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

impl Hostinfo {
    /// Generate empty Hostinfo
    /// 
    /// Generates an empty Hostinfo.
    pub fn empty() -> Hostinfo {
        Hostinfo {
            name: "".to_string(),
            icon: "".to_string(),
            uuid: "".to_string(),
            secured: false,
            size: 0,
            origin: "".to_string(),
            albums: Vec::new(),
            owners: Vec::new()
        }
    }


    /// Generate Hostinfo
    /// 
    /// Generates a Hostinfo based on a [Config].
    pub fn generate(cfg: Config) -> io::Result<Hostinfo> {
        let albums = Hostinfo::read_albums(cfg.root.as_str())?;
        let mut hostinfo = Hostinfo::from(cfg);
        hostinfo.albums = albums.0;
        hostinfo.size = albums.1;
        Ok(hostinfo)
    }

    /// Read Albums
    /// 
    /// Reads a given directory and returns it as a [Vec] of [Album]s
    pub fn read_albums(path: impl AsRef<Path>) -> io::Result<(Vec<Album>, u128)> {
        // Total Size of Albums
        let mut totsize: u128 = 0;

        // Read given Cyrkensia root
        let albums: Vec<Album> = fs::read_dir(path.as_ref())?

        // Only use working entries and get directory path and name
        .filter_map(|x| {
            if let Ok(dir) = x {
                if dir.path().is_dir() {
                    return Some((dir.path(), dir.file_name()));
                }
            }
            None
        })

        // Create Album instances
        .filter_map(|y| {
            // Load Metadata
            if let Ok(meta) = Metadata::load(y.0.join(".metadata.json")) {

                // Just for safety. Safely converts the OsString to a String.
                if let Some(fname) = y.1.to_str() {

                    // Read files and total size
                    if let Ok(res) = Hostinfo::list_files(fname) {

                        // Add size and return Album
                        totsize += res.1;
                        return Some(Album {
                            name: meta.name,
                            cover: meta.cover,
                            path: fname.to_string(),
                            artists: meta.artists,
                            files: res.0
                        });
                    }  
                }
            }
            None
        })

        // Collect and Return
        .collect();
        Ok((albums, totsize))
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
            if let Ok(entry) = x {

                // Filter out non-files
                if entry.path().is_file() {

                    // Get successful Metadata
                    if let Ok(fmeta) = entry.metadata() {

                        // Return Path and Length
                        return Some((entry.path(), fmeta.len()));
                    }
                }               
            }
            None
        })
        
        // Convert PathBuf to String
        .filter_map(|y| {
            if let Ok(pathstr) = y.0.into_os_string().into_string() {
                // Add filesize to total buffer
                allsize += y.1 as u128;
                return Some(pathstr);
            }
            None
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