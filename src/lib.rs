// TODO: Add description here and instructions for using the library




// TODO: Add exports (*.rs files)




/// Metadata about Cyrkensia
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