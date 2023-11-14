use anyhow::{anyhow, Error as AnyError};
use clap::Parser;
use geo::geometry::Coord;
use std::{path::PathBuf, str::FromStr};

/// Generate point-to-point terrain profiles.
#[derive(Parser, Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct Cli {
    /// Directory elevation tiles.
    #[arg(short, long)]
    pub tile_dir: PathBuf,

    /// Maximum path incremental step size, in meters.
    #[arg(short, long, default_value_t = 90.0)]
    pub max_step: f64,

    /// Start "lat,lon,alt", where 'alt' is meters above ground.
    #[arg(long)]
    pub start: LatLonAlt,

    /// Destination "lat,lon,alt", where 'alt' is meters above ground.
    #[arg(long)]
    pub end: LatLonAlt,

    /// Signal frequency (Hz).
    #[arg(long, short)]
    pub frequency: f64,
}

#[derive(Clone, Debug, Copy)]
pub struct LatLonAlt(pub Coord<f64>, pub f64);

impl FromStr for LatLonAlt {
    type Err = AnyError;
    fn from_str(s: &str) -> Result<Self, AnyError> {
        let (lat_str, lon_str, alt_str) = {
            let idx = s
                .find(',')
                .ok_or_else(|| anyhow!("not a valid lat,lon,alt"))?;
            let (lat_str, lon_alt_str) = s.split_at(idx);
            let idx = lon_alt_str[1..]
                .find(',')
                .ok_or_else(|| anyhow!("not a valid lat,lon,alt"))?;
            let (lon_str, alt_str) = lon_alt_str[1..].split_at(idx);
            (lat_str, lon_str, &alt_str[1..])
        };
        let lat = f64::from_str(lat_str)?;
        let lon = f64::from_str(lon_str)?;
        let alt = f64::from_str(alt_str)?;
        Ok(Self(Coord { y: lat, x: lon }, alt))
    }
}
