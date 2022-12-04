//! # Cyrkensia
//! ![Build (Windows)](https://github.com/Stridsvagn69420/Cyrkensia/actions/workflows/build-windows.yml/badge.svg?branch=master)
//! ![Build (Linux)](https://github.com/Stridsvagn69420/Cyrkensia/actions/workflows/build-linux.yml/badge.svg?branch=master)
//! ![Clippy](https://github.com/Stridsvagn69420/Cyrkensia/actions/workflows/cargo-clippy.yml/badge.svg?branch=master)
//! ![docs.rs](https://docs.rs/cyrkensia/badge.svg)
//! ![Version](https://img.shields.io/crates/v/cyrkensia.svg)
//! ![License](https://img.shields.io/crates/l/cyrkensia.svg)
//! ![Stars](https://img.shields.io/github/stars/Stridsvagn69420/Cyrkensia.svg)
//! ![Downloads](https://img.shields.io/crates/d/cyrkensia.svg)
//! 
//! NOTE: This documentation focuses on Cyrkensia as a library.
//! If you're searching for Cyrkensia as a binary, see the [crates.io](https://crates.io/crates/cyrkensia) or [GitHub](https://github.com/Stridsvagn69420/Cyrkensia) page for more.
//! 
//! ## Features
//! By default, the `server` feature is enabled. The latter is only relevant for people who write a custom Cyrkensia server.
//! You can disable `server` with this:
//! ```toml
//! cyrkensia = { version = "1", default-features = false }
//! ```
//! 
//! ## Examples
//! 
//! Reading a Config and generating its Hostinfo
//! ```no_run
//! use cyrkensia::{Hostinfo, Config, Artist};
//! 
//! // Load the config file
//! let config = Config::load_file("config.json").unwrap();
//! // Load the artists
//! let artists = Artist::read_multiple(&config.root).unwrap();
//! // Generate the corresponding Hostinfo
//! let mut hostinfo = Hostinfo::generate(&config, &artists).unwrap();
//! // Optionally set the origin
//! hostinfo.set_origin("https://foo.bar/my-hostinfo.json".to_string());
//! ```

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
	pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
	pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
	pub const WIKI_HELP_URL: &str = concat!(env!("CARGO_PKG_REPOSITORY"), "/wiki/Troubleshooting");
	pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
	pub const LICENSE: &str = "EUPL-1.2";
	pub const LICENSE_RICH: &str = "European Union Public License v1.2";
	pub const LICENSE_URL: &str = "https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12";
	pub const CARGO_VERSION: &str = env!("CARGO_VERSION");
	pub const RUSTC_VERSION: &str = env!("RUSTC_VERSION");
	pub const COMPILE_DATE: &str = env!("COMPILE_DATE");
	pub const TARGET: &str = env!("TARGET");
	pub const USERAGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
	pub const CONFIG_ENVVAR: &str = "CYRKENSIA_CONFIG";
	pub const CONFIG_PATH: &str = concat!(".config/", env!("CARGO_PKG_NAME"), "/config.json");
	pub const USERS_ENVVAR: &str = "CYRKENSIA_USERS";
	pub const USERS_PATH: &str = concat!(".config/", env!("CARGO_PKG_NAME"), "/users.json");
}

#[cfg(feature = "server")]
/// Server Routes
/// 
/// This submodule contains middlewares and other tools needed only for the Cyrkensia server.
pub mod server;

#[cfg(feature = "accounts")]
/// Account database
/// 
/// Submodule containing the [Account](accounts::Account) struct and related cryptographic and wrapper functions.
pub mod account;
