library;

import 'dart:math' as math;

/// 6x6 symbol grid
const List<List<String>> digipinGrid = [
  ['I', 'A', 'B', 'C', 'D', 'E'],
  ['G', 'H', 'J', 'K', 'L', 'M'],
  ['N', 'P', 'Q', 'R', 'S', 'T'],
  ['U', 'r', 'W', 'X', 'Y', 'Z'],
  ['a', 'b', '9', 'd', 'V', 'F'],
  ['2', '3', '4', '5', '6', '7'],
];

/// WGS-84 sphere (meters)
const double _r = 6378137.0;

/// Mercator latitude limit (degrees)
const double maxLat = 85.05112878;

const double _pi = math.pi;

/// Precomputed Mercator world bounds (meters)
class _MercBounds {
  final double minX, maxX, minY, maxY;
  const _MercBounds({
    required this.minX,
    required this.maxX,
    required this.minY,
    required this.maxY,
  });
}

final _mercBounds = _computeMercBounds();

_MercBounds _computeMercBounds() {
  final maxLatRad = (maxLat * _pi / 180.0);
  final minY = -_r * math.log(math.tan(_pi / 4 + maxLatRad / 2));
  final maxY =  _r * math.log(math.tan(_pi / 4 + maxLatRad / 2));
  return _MercBounds(
    minX: -_pi * _r,
    maxX:  _pi * _r,
    minY: minY,
    maxY: maxY,
  );
}

double _clamp(double v, double lo, double hi) =>
    v < lo ? lo : (v > hi ? hi : v);

double _normalizeLon(double lon) {
  // Normalize to [-180, 180), with 180 mapped to -180
  final double wrapped =
      (((lon + 180.0) % 360.0) + 360.0) % 360.0 - 180.0;
  return wrapped == 180.0 ? -180.0 : wrapped;
}

double _lonToX(double lonDeg) => _r * (_normalizeLon(lonDeg) * _pi / 180.0);
double _xToLon(double x) => _normalizeLon((x / _r) * 180.0 / _pi);

double _latToY(double latDeg) {
  final phi = (_clamp(latDeg, -maxLat, maxLat)) * _pi / 180.0;
  return _r * math.log(math.tan(_pi / 4.0 + phi / 2.0));
}

double _yToLat(double y) {
  final phi = 2.0 * math.atan(math.exp(y / _r)) - _pi / 2.0;
  return phi * 180.0 / _pi;
}

/// Encode lat/lon to a square-cell global code.
/// [levels] = number of symbols (default 10).
/// Returns grouped string (e.g., "ABCD-EFGH-IJ" for 10).
String getDigiPin(double lat, double lon, [int levels = 10]) {
  if (!lat.isFinite || !lon.isFinite) {
    throw ArgumentError('lat/lon must be finite numbers');
  }
  lat = _clamp(lat, -maxLat, maxLat);
  lon = _normalizeLon(lon);

  // to Mercator meters
  double x = _lonToX(lon);
  double y = _latToY(lat);

  const double eps = 1e-9;
  double minX = _mercBounds.minX, maxX = _mercBounds.maxX;
  double minY = _mercBounds.minY, maxY = _mercBounds.maxY;

  x = math.min(math.max(x, minX + eps), maxX - eps);
  y = math.min(math.max(y, minY + eps), maxY - eps);

  final StringBuffer code = StringBuffer();

  for (int level = 1; level <= levels; level++) {
    final double xDiv = (maxX - minX) / 6.0;
    final double yDiv = (maxY - minY) / 6.0;

    final int rowRaw = 5 - ((y - minY) / yDiv).floor(); // top row = 0
    final int colRaw = ((x - minX) / xDiv).floor();
    final int row = rowRaw.clamp(0, 5);
    final int col = colRaw.clamp(0, 5);

    code.write(digipinGrid[row][col]);

    final double newMaxY = minY + yDiv * (6 - row);
    final double newMinY = minY + yDiv * (5 - row);
    minX = minX + xDiv * col;
    final double newMaxX = minX + xDiv;

    minY = newMinY;
    maxY = newMaxY;
    maxX = newMaxX;
  }

  final raw = code.toString();
  if (raw.length == 10) {
    // 4-4-2 grouping
    return '${raw.substring(0, 4)}-${raw.substring(4, 8)}-${raw.substring(8)}';
  }

  return _groupEvery4(raw);
}

String _groupEvery4(String s) {
  final re = RegExp(r'.{1,4}');
  final parts = re.allMatches(s).map((m) => m.group(0)!).toList();
  return parts.join('-');
}

/// Simple lat/lon holder
class LatLng {
  final double latitude;
  final double longitude;
  const LatLng(this.latitude, this.longitude);
  @override
  String toString() => 'LatLng(lat: $latitude, lon: $longitude)';
}

/// Decode a code back to the center lat/lon of its cell.
LatLng getLatLngFromDigiPin(String digiPin) {
  final pin = digiPin.replaceAll('-', '');
  if (pin.isEmpty) {
    throw ArgumentError('Invalid DIGIPIN');
  }

  // Build a lookup map char -> (row, col)
  final Map<String, GridPos> lookup = {};
for (int r = 0; r < 6; r++) {
  for (int c = 0; c < 6; c++) {
    lookup[digipinGrid[r][c]] = GridPos(r, c);
  }
}


  double minX = _mercBounds.minX, maxX = _mercBounds.maxX;
  double minY = _mercBounds.minY, maxY = _mercBounds.maxY;

  for (int i = 0; i < pin.length; i++) {
    final ch = pin[i];
    final rc = lookup[ch];
    if (rc == null) throw ArgumentError("Invalid character '$ch'");
    final ri = rc.lat;
    final ci = rc.lon;


    final double xDiv = (maxX - minX) / 6.0;
    final double yDiv = (maxY - minY) / 6.0;

    final double y1 = maxY - yDiv * (ri + 1);
    final double y2 = maxY - yDiv * ri;
    final double x1 = minX + xDiv * ci;
    final double x2 = x1 + xDiv;

    minY = y1;
    maxY = y2;
    minX = x1;
    maxX = x2;
  }

  final double cx = (minX + maxX) / 2.0;
  final double cy = (minY + maxY) / 2.0;

  return LatLng(_yToLat(cy), _xToLon(cx));
}

/// Convenience: approximate cell size (meters) for a given code length.
double approxCellSizeMeters(int levels) {
  final world = 2 * math.pi * _r;
  return world / math.pow(6, levels);
}

class GridPos {
  final int lat;
  final int lon;
  const GridPos(this.lat, this.lon);
}
