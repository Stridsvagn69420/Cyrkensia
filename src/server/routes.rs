use std::time::Instant;
use actix_web::{web, Responder, HttpRequest};
use super::{CyrkensiaState, responses, uri_noquery};
use crate::{Hostinfo, Artist};

/// Hostinfo Route
/// 
/// Route for serving a [Hostinfo]. Server needs [CyrkensiaState] in `.app_data()` for this.
pub async fn hostinfo(req: HttpRequest, data: web::Data<CyrkensiaState>) -> impl Responder {
    // Get config
    let Some(delay) = data.config.max_age else {
        // Ad hoch Hostinfo
        let Ok(artists) = Artist::read_multiple(&data.config.root) else {
            return responses::server_500(Some("Failed to generate hostinfo"));
        };
        let Ok(resp) = responses::hostinfo_json(&data.config, &artists) else {
            return responses::server_500(Some("Failed to generate hostinfo"));
        };
        return resp;
    };

    // Get last update timestamp and cached hostinfo and artists
    let Ok(mut last_updated) = data.last_updated.lock() else {
        return responses::server_500(None::<String>);
    };
    let Ok(mut hostinfo) = data.hostinfo.lock() else {
        return responses::server_500(None::<String>);
    };
    let Ok(mut artists) = data.artists.lock() else {
        return responses::server_500(None::<String>);
    };

    // Update Cache if expired
    if last_updated.elapsed().as_secs() >= delay {
        // Read updated artists
        let Ok(new_artists) = Artist::read_multiple(&data.config.root) else {
            return responses::server_500(Some("Failed to update hostinfo"));
        };

        // Generate new Hostinfo
        let Ok(new_hostinfo) = Hostinfo::generate(&data.config, &artists) else {
            return responses::server_500(Some("Failed to update hostinfo"));
        };

        // Update Hostinfo and Timestamp
        *artists = new_artists;
        *hostinfo = new_hostinfo;
        *last_updated = Instant::now();
    }

    // Set Origin URL
    let mut final_hostinfo = hostinfo.clone();
    final_hostinfo.set_origin(uri_noquery(req.uri()));

    // Return final result
    let Ok(finalres) = responses::hostinfo_data(&final_hostinfo) else {
        return responses::server_500(Some("Failed to generate hostinfo"));
    };
    finalres
}