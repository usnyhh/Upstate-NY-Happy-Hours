//! Approximate location of the person running the client, from their public IP.
//!
//! City-level accuracy at best (worse behind VPNs and some mobile carriers), which
//! is enough to preselect the nearest city. The lookup sends the user's IP to a
//! third-party service, so it only ever seeds a default the user can change.

use serde::Deserialize;

use crate::http;

/// HTTPS, keyless, and CORS-enabled (so it also works from the wasm build).
/// The keyless tier is rate-limited, which one lookup per launch stays well under.
const LOOKUP_URL: &str = "https://ipinfo.io/json";

#[derive(Debug, Clone, Copy)]
pub struct Coords {
    pub lat: f64,
    pub lon: f64,
}

impl Coords {
    /// Great-circle distance in kilometres (haversine).
    pub fn distance_km(self, other: Coords) -> f64 {
        const EARTH_RADIUS_KM: f64 = 6371.0;
        let (lat1, lat2) = (self.lat.to_radians(), other.lat.to_radians());
        let dlat = lat2 - lat1;
        let dlon = (other.lon - self.lon).to_radians();
        let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
        2.0 * EARTH_RADIUS_KM * a.sqrt().asin()
    }
}

#[derive(Deserialize)]
struct Response {
    /// `"lat,lon"`; absent for e.g. private/bogon IPs.
    loc: Option<String>,
}

/// Look up the user's approximate coordinates from their public IP.
pub async fn locate() -> Result<Coords, String> {
    let body: Response = http::get_json(LOOKUP_URL)
        .await
        .map_err(|e| format!("IP geolocation failed: {e}"))?;
    let loc = body.loc.ok_or("IP geolocation response had no coordinates")?;
    parse_loc(&loc).ok_or_else(|| format!("Unparseable IP geolocation coordinates: {loc:?}"))
}

/// Parse ipinfo's `"lat,lon"` string.
fn parse_loc(loc: &str) -> Option<Coords> {
    let (lat, lon) = loc.split_once(',')?;
    Some(Coords {
        lat: lat.trim().parse().ok()?,
        lon: lon.trim().parse().ok()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_loc() {
        let c = parse_loc("43.1548,-77.6156").unwrap();
        assert_eq!((c.lat, c.lon), (43.1548, -77.6156));
        assert!(parse_loc("garbage").is_none());
        assert!(parse_loc("1,x").is_none());
    }
}
