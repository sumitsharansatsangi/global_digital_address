// Package global_digital_address provides a square-cell Web Mercator encoder/decoder for globally
// unique grid codes.
//
// API:
//   - Encode(lat, lon float64, levels int) string
//   - Decode(code string) (lat, lon float64, levels int, err error)
//   - ApproxCellSizeMeters(levels int) float64
//   - Vars: Alphabet, MaxLat
//
// A DigiPin is produced by repeatedly subdividing the Web Mercator world into a
// 6x6 grid and indexing each cell using a fixed 36-symbol alphabet. The default
// code length is 10 symbols (grouped like "ABCD-EFGH-IJ").
//
// Coordinates use WGS-84 degrees. Internally, conversions use spherical Web
// Mercator (EPSG:3857) with latitude clamped to ±85.05112878°.
package global_digital_address
