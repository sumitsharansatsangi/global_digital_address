/**
 * global_digital_address
 * Square-cell Web Mercator encoder/decoder for globally unique grid codes.
 * API:
 *  - getDigiPin(lat, lon, levels?)
 *  - getLatLngFromDigiPin(code)
 *  - constants: DIGIPIN_GRID, MAX_LAT
 */
export declare const DIGIPIN_GRID: string[][];
export declare const MAX_LAT = 85.05112878;
/**
 * Encode lat/lon to a square-cell global code.
 * @param lat Latitude in degrees (WGS-84)
 * @param lon Longitude in degrees (WGS-84)
 * @param levels Number of symbols; default 10
 * @returns grouped string, e.g. "ABCD-EFGH-IJ"
 */
export declare function getDigiPin(lat: number, lon: number, levels?: number): string;
/**
 * Decode a code back to center lat/lon of its cell.
 */
export declare function getLatLngFromDigiPin(digiPin: string): {
    latitude: number;
    longitude: number;
};
/** Convenience: compute approximate cell size (meters) for a given code length */
export declare function approxCellSizeMeters(levels: number): number;
