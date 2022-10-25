use serde::{Serialize, Deserialize};
use super::Owner;

/// Configuration
/// 
/// The server configuration used for Cyrkensia.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Name
    /// 
    /// The display name of the repository.
    pub name: String,

    /// File root
    /// 
    /// The file root where all albums are stored.
    pub root: String,

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
    pub owners: Vec<Owner>
}

// TODO: Add trait implementations and other useful functions to the struct