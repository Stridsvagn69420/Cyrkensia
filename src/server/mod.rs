use crate::{Hostinfo, Config};
use std::sync::Mutex;
use std::time::Instant;
use std::io;

/// Middlewares
/// 
/// Submodule containing Cyrkensia middlewares.
pub mod middleware;

/// Redirects
/// 
/// Submodule containing Cyrkensia redirects.
pub mod redirect;


/// Routes
/// 
/// Submodule containing Cyrkensia routes.
pub mod routes;

/// Cyrkensia State
/// 
/// State for the Actix-Web server. Used in [routes].
pub struct CyrkensiaState {
    /// Config
    /// 
    /// The loaded Config (read-only)
    pub config: Config,

    /// Hostinfo
    /// 
    /// The latest generated [Hostinfo].
    /// Only used if caching is activated.
    pub hostinfo: Mutex<Option<Hostinfo>>,

    /// Last Hostinfo Update
    /// 
    /// The [timestamp](Instant) when the [hostinfo] was last updated.
    /// `.elapsed().as_secs()` will be used to compare it with the `max_age` in the [Config].
    /// Only used if caching is activated.
    pub last_updated: Mutex<Option<Instant>>
}

impl CyrkensiaState {
    /// Constructur
    /// 
    /// Creates a new [CyrkensiaState] with given [Config].
    pub fn new(cfg: Config) -> io::Result<CyrkensiaState> {
        if cfg.max_age.is_some() {
            // State with caching
            let hostinfo = Hostinfo::generate(cfg.clone())?;
            return Ok(CyrkensiaState {
                last_updated: Mutex::new(Some(Instant::now())),
                hostinfo: Mutex::new(Some(hostinfo)),
                config: cfg
            });
        }

        // State without caching
        Ok(CyrkensiaState {
            hostinfo: Mutex::new(None),
            last_updated: Mutex::new(None),
            config: cfg,
        })
    }
}