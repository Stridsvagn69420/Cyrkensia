use std::cmp::PartialEq;



// TODO: Add description here and instructions for using the library



/// Owner struct
mod owner;
pub use self::owner::Owner;

/// Config struct
mod config;
pub use self::config::Config;

/// Metadata struct
mod metadata;
pub use self::metadata::Metadata;

/// Artist struct
mod artist;
pub use self::artist::Artist;

/// Album struct
mod album;
pub use self::album::Album;

/// Hostinfo struct
mod hostinfo;
pub use self::hostinfo::Hostinfo;

/// Meta about Cyrkensia
/// 
/// This little module just contains information about this crate.
pub mod meta {
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const NAME: &str = env!("CARGO_PKG_NAME");
    pub const NAME_RICH: &str = "Cyrkensia";
    pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
    pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
    pub const LICENSE: &str = "EUPL-1.2";
    pub const LICENSE_RICH: &str = "European Union Public License v1.2";
    pub const LICENSE_URL: &str = "https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12";
    pub const CARGO_VERSION: &str = env!("CARGO_VERSION");
    pub const RUSTC_VERSION: &str = env!("RUSTC_VERSION");
    pub const COMPILE_DATE: &str = env!("COMPILE_DATE");
    pub const TARGET: &str = env!("TARGET");
    pub const USERAGENT: &str = concat!(
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION")
    );
}

/// Server Routes
/// 
/// This submodule (only visible locally) contains routes and other tools needed only for the Cyrkensia server.
pub(crate) mod server;

/// Remove from [Vec]
/// 
/// Removes an element from a [Vec], if it exists already, without returning a new [Vec].
pub(crate) fn remove_vec<T: PartialEq>(arr: &mut Vec<T>, elm: T) {
    if let Some(i) = arr.iter().position(|x| *x == elm) {
        arr.remove(i);
    }
}

/// Add to [Vec]
/// 
/// Adds an element to a [Vec], if the element doesn't exist yet, without returning a new [Vec].
pub(crate) fn add_vec<T: PartialEq>(arr: &mut Vec<T>, elm: T) {
    if !arr.contains(&elm) {
        arr.push(elm);
    }
}