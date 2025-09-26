//! global_digital_address
//! Square-cell Web Mercator encoder/decoder for globally unique grid codes.
//!
//! API:
//!   - `get_digi_pin(lat, lon, levels)` -> String
//!   - `get_lat_lng_from_digipin(code)` -> Result<LatLng, Error>
//!   - constants: `DIGIPIN_GRID`, `MAX_LAT`
//!   - `approx_cell_size_meters(levels)`
//!
//! Notes:
//!   - Uses Web Mercator (EPSG:3857) math.
//!   - Longitude normalized to (-180, 180]; latitude clamped to ±MAX_LAT.
//!   - Grid subdivision is 6x6 per level.

use core::f64::consts::PI;

/// WGS-84 Earth radius used by Web Mercator (meters)
const R: f64 = 6_378_137.0;

/// Mercator latitude clamp (degrees)
pub const MAX_LAT: f64 = 85.051_128_78;

/// 6×6 symbol grid (rows top→bottom, columns left→right)
pub const DIGIPIN_GRID: [[char; 6]; 6] = [
    ['I', 'A', 'B', 'C', 'D', 'E'],
    ['G', 'H', 'J', 'K', 'L', 'M'],
    ['N', 'P', 'Q', 'R', 'S', 'T'],
    ['U', 'r', 'W', 'X', 'Y', 'Z'],
    ['a', 'b', '9', 'd', 'V', 'F'],
    ['2', '3', '4', '5', '6', '7'],
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LatLng {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("lat/lon must be finite numbers")]
    NonFiniteInput,
    #[error("invalid DIGIPIN")]
    InvalidPin,
    #[error("invalid character '{0}' in DIGIPIN")]
    InvalidChar(char),
}

#[inline]
fn clamp(v: f64, lo: f64, hi: f64) -> f64 { v.max(lo).min(hi) }

/// Normalize longitude to (-180, 180]; returns -180 instead of +180.
#[inline]
fn normalize_lon(lon: f64) -> f64 {
    let x = ((lon + 180.0) % 360.0 + 360.0) % 360.0 - 180.0;
    if x == 180.0 { -180.0 } else { x }
}

#[inline] fn lon_to_x(lon_deg: f64) -> f64 { R * normalize_lon(lon_deg).to_radians() }
#[inline] fn x_to_lon(x: f64) -> f64 { normalize_lon((x / R).to_degrees()) }

#[inline]
fn lat_to_y(lat_deg: f64) -> f64 {
    let phi = clamp(lat_deg, -MAX_LAT, MAX_LAT).to_radians();
    R * f64::ln(f64::tan(PI / 4.0 + phi / 2.0))
}

#[inline]
fn y_to_lat(y: f64) -> f64 {
    let phi = 2.0 * f64::atan(f64::exp(y / R)) - PI / 2.0;
    phi.to_degrees()
}

struct MercBounds { min_x: f64, max_x: f64, min_y: f64, max_y: f64 }

fn merc_bounds() -> MercBounds {
    let min_x = -PI * R;
    let max_x =  PI * R;
    let max_phi = MAX_LAT.to_radians() / 2.0 + PI / 4.0;
    let max_y = R * f64::ln(f64::tan(max_phi));
    let min_y = -max_y;
    MercBounds { min_x, max_x, min_y, max_y }
}

/// Encode lat/lon to a square-cell global code.
/// - `levels`: number of symbols (default = 10)
pub fn get_digi_pin(lat: f64, lon: f64, levels: usize) -> Result<String, Error> {
    if !lat.is_finite() || !lon.is_finite() { return Err(Error::NonFiniteInput); }

    let lat = clamp(lat, -MAX_LAT, MAX_LAT);
    let lon = normalize_lon(lon);

    let mut x = lon_to_x(lon);
    let mut y = lat_to_y(lat);

    let eps = 1e-9_f64;
    let MercBounds { mut min_x, mut max_x, mut min_y, mut max_y } = merc_bounds();

    // keep strictly inside bounds
    x = x.max(min_x + eps).min(max_x - eps);
    y = y.max(min_y + eps).min(max_y - eps);

    let mut code = String::with_capacity(levels);

    for _ in 0..levels {
        let x_div = (max_x - min_x) / 6.0;
        let y_div = (max_y - min_y) / 6.0;

        // top row = 0
        let row_raw = 5.0 - ((y - min_y) / y_div).floor();
        let col_raw = ((x - min_x) / x_div).floor();

        let row = clamp(row_raw, 0.0, 5.0) as usize;
        let col = clamp(col_raw, 0.0, 5.0) as usize;

        code.push(DIGIPIN_GRID[row][col]);

        // refine bounds
        let new_max_y = min_y + y_div * (6.0 - row as f64);
        let new_min_y = min_y + y_div * (5.0 - row as f64);

        min_x = min_x + x_div * (col as f64);
        let new_max_x = min_x + x_div;

        min_y = new_min_y; max_y = new_max_y; max_x = new_max_x;
    }

    Ok(group_code(&code))
}

/// Decode a DIGIPIN back to the center lat/lon of its cell.
pub fn get_lat_lng_from_digipin(digipin: &str) -> Result<LatLng, Error> {
    let pin: String = digipin.chars().filter(|&ch| ch != '-').collect();
    if pin.is_empty() { return Err(Error::InvalidPin); }

    let MercBounds { mut min_x, mut max_x, mut min_y, mut max_y } = merc_bounds();

    for ch in pin.chars() {
        let (ri, ci) = lookup_grid(ch).ok_or(Error::InvalidChar(ch))?;

        let x_div = (max_x - min_x) / 6.0;
        let y_div = (max_y - min_y) / 6.0;

        // rows counted from top
        let y1 = max_y - y_div * ((ri + 1) as f64);
        let y2 = max_y - y_div * (ri as f64);
        let x1 = min_x + x_div * (ci as f64);
        let x2 = x1 + x_div;

        min_y = y1; max_y = y2;
        min_x = x1; max_x = x2;
    }

    let cx = (min_x + max_x) / 2.0;
    let cy = (min_y + max_y) / 2.0;

    Ok(LatLng { latitude: y_to_lat(cy), longitude: x_to_lon(cx) })
}

/// Approximate cell size (meters) for a given code length.
/// world width / 6^levels
pub fn approx_cell_size_meters(levels: usize) -> f64 {
    let world = 2.0 * PI * R;
    world / 6f64.powi(levels as i32)
}

/// Group into "AAAA-BBBB-CC" when len==10; else groups of 4.
fn group_code(raw: &str) -> String {
    if raw.len() == 10 {
        format!("{}-{}-{}", &raw[0..4], &raw[4..8], &raw[8..10])
    } else {
        let mut out = String::with_capacity(raw.len() + raw.len()/4);
        for (i, ch) in raw.chars().enumerate() {
            if i > 0 && i % 4 == 0 { out.push('-'); }
            out.push(ch);
        }
        out
    }
}

#[inline]
fn lookup_grid(ch: char) -> Option<(usize, usize)> {
    for (r, row) in DIGIPIN_GRID.iter().enumerate() {
        for (c, &sym) in row.iter().enumerate() {
            if sym == ch { return Some((r, c)); }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lon_normalization() {
        assert_eq!(super::normalize_lon(180.0), -180.0);
        assert!((super::normalize_lon(181.0) - -179.0).abs() < 1e-12);
        assert!((super::normalize_lon(-181.0) - 179.0).abs() < 1e-12);
    }

    #[test]
    fn approx_size_levels() {
        let l5 = approx_cell_size_meters(5);
        let l6 = approx_cell_size_meters(6);
        assert!(l6 < l5);
    }

    #[test]
    fn roundtrip_delhi_like() {
        let lat = 28.6139_f64;
        let lon = 77.2090_f64;

        let code = get_digi_pin(lat, lon, 10).unwrap();
        let ll = get_lat_lng_from_digipin(&code).unwrap();

        let dist_lat = (ll.latitude - lat).abs();
        let dist_lon = (ll.longitude - lon).abs();
        assert!(dist_lat < 0.05);
        assert!(dist_lon < 0.05);
    }

    #[test]
    fn grouping_rules() {
        assert_eq!(super::group_code("ABCDEFGHIJ"), "ABCD-EFGH-IJ");
        assert_eq!(super::group_code("ABCDEFGH"), "ABCD-EFGH");
        assert_eq!(super::group_code("ABCDEF"), "ABCD-EF");
        assert_eq!(super::group_code("ABCD"), "ABCD");
        assert_eq!(super::group_code("ABC"), "ABC");
    }

    #[test]
    fn invalid_char() {
        let bad = "AAAA$";
        let err = get_lat_lng_from_digipin(bad).unwrap_err();
        match err {
            Error::InvalidChar('$') => {}
            _ => panic!("unexpected error"),
        }
    }
}
