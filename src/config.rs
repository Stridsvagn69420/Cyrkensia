use serde::{Serialize, Deserialize};
use serde_json::from_str;
use std::cmp::PartialEq;
use std::convert::From;
use std::io;
use std::fs;
use std::path::Path;
use super::{Owner, Hostinfo};

/// Configuration
/// 
/// The server configuration used for Cyrkensia.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// Name
    /// 
    /// The display name of the repository.
    pub name: String,

    /// File roots
    /// 
    /// The file roots where all albums are stored. 
    pub root: Vec<String>,

    /// UUIDv4
    /// 
    /// The UUID of the repository.
    pub uuid: String,

    /// Icon
    /// 
    /// The Rich Presence asset key used for displaying the server.
    pub icon: String,

    /// Htpasswd file
    /// 
    /// The path to the .htpasswd file for HTTP-Basic Authentication. If none is present, HTTP Basic Auth will be disabled.
    pub htpasswd: Option<String>,

    /// Bind address
    /// 
    /// The IP address to bind to, e.g. `127.0.0.1`, `0.0.0.0:80` or a Unix socket (Unix only).
    pub bindaddr: String,

    /// TLS Certificate (Optional)
    /// 
    /// The Path to the TLS certificate. TLS will only be activated if both certificate and key are provided.
    pub tlscert: Option<String>,

    /// TLS Key (Optional)
    /// 
    /// The path to the TLS key. TLS will only be activated if both certificate and key are provided.
    pub tlskey: Option<String>,

    /// Owners
    /// 
    /// List of repository [maintainers](Owner)
    pub owners: Vec<Owner>,

    /// Maximum Age
    /// 
    /// The maximum age of the [Hostinfo] in milliseconds as a [u64]. If [None], the Hostinfo will always be regenerated when its route is accessed.
    /// This basically activates caching.
    pub max_age: Option<u64>
}

impl Config {
    /// Load Config File
    /// 
    /// Loads a config from a file in the filesystem.
    pub fn load_file(path: impl AsRef<Path>) -> io::Result<Config> {
        let rawfile = fs::read_to_string(path)?;
        Ok(from_str(rawfile.as_str())?)
    }

    /// Load Config JSON
    /// 
    /// Loads a config from an already existing &[str].
    pub fn load_json(data: &str) -> io::Result<Config> {
        Ok(from_str(data)?)
    }
}

impl From<Hostinfo> for Config {
    fn from(x: Hostinfo) -> Config {
        Config {
            name: x.name,
            root: Vec::new(),
            uuid: x.uuid,
            icon: x.icon,
            htpasswd: None,
            bindaddr: "".to_string(),
            tlscert: None,
            tlskey: None,
            owners: x.owners,
            max_age: None
        }
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.root == other.root &&
        self.uuid == other.uuid &&
        self.icon == other.icon &&
        self.htpasswd == other.htpasswd &&
        self.bindaddr == other.bindaddr &&
        self.tlscert == other.tlscert &&
        self.tlskey == other.tlskey &&
        self.owners == other.owners
    }
}