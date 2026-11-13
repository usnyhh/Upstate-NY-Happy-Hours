//! The cities a user can browse, and picking a default one from their location.

use std::fmt;

use crate::geolocate::Coords;

/// Cities selectable from the navbar's pick list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum City {
    Albany,
    Buffalo,
    Ithaca,
    Rochester,
    Syracuse,
}

impl City {
    pub const ALL: [City; 5] = [
        City::Albany,
        City::Buffalo,
        City::Ithaca,
        City::Rochester,
        City::Syracuse,
    ];

    /// Only preselect a city if the user is within this distance of it, so someone
    /// in, say, New York City or out of state isn't silently dropped into one.
    const PRESELECT_RADIUS_KM: f64 = 150.0;

    pub fn name(self) -> &'static str {
        match self {
            City::Albany => "Albany",
            City::Buffalo => "Buffalo",
            City::Ithaca => "Ithaca",
            City::Rochester => "Rochester",
            City::Syracuse => "Syracuse",
        }
    }

    /// Approximate city-centre coordinates.
    pub fn coords(self) -> Coords {
        let (lat, lon) = match self {
            City::Albany => (42.6526, -73.7562),
            City::Buffalo => (42.8864, -78.8784),
            City::Ithaca => (42.4440, -76.5019),
            City::Rochester => (43.1566, -77.6088),
            City::Syracuse => (43.0481, -76.1474),
        };
        Coords { lat, lon }
    }

    /// The closest city to `here`, if any is within `PRESELECT_RADIUS_KM`.
    pub fn nearest(here: Coords) -> Option<City> {
        City::ALL
            .into_iter()
            .map(|city| (city, here.distance_km(city.coords())))
            .filter(|&(_, km)| km <= Self::PRESELECT_RADIUS_KM)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(city, _)| city)
    }
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(lat: f64, lon: f64) -> Coords {
        Coords { lat, lon }
    }

    #[test]
    fn nearest_city_within_radius() {
        // Downtown Rochester, Cornell, Saratoga Springs (~50 km from Albany).
        assert_eq!(City::nearest(at(43.157, -77.615)), Some(City::Rochester));
        assert_eq!(City::nearest(at(42.4534, -76.4735)), Some(City::Ithaca));
        assert_eq!(City::nearest(at(43.0831, -73.7846)), Some(City::Albany));
    }

    #[test]
    fn no_city_when_too_far() {
        // New York City and San Francisco.
        assert_eq!(City::nearest(at(40.7128, -74.0060)), None);
        assert_eq!(City::nearest(at(37.7749, -122.4194)), None);
    }
}
