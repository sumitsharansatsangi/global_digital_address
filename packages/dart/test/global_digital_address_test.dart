import 'dart:math' as math;
import 'package:test/test.dart';
import 'package:global_digital_address/global_digital_address.dart';

bool _nearlyEqual(double a, double b, [double tol = 1e-6]) =>
    (a - b).abs() <= tol;

void main() {
  test('roundtrip known coordinate (New Delhi)', () {
    final lat = 28.6139, lon = 77.2090;
    final code = getDigiPin(lat, lon, 10);
    final center = getLatLngFromDigiPin(code);

    // Center should be close to original (within cell size).
    final approx = approxCellSizeMeters(10);
    // Convert meters to ~degrees at equator for a loose check:
    final metersPerDeg = 111320.0;
    final tolDeg = approx / metersPerDeg;

    expect(_nearlyEqual(center.latitude, lat, tolDeg), isTrue,
        reason: 'lat mismatch: got ${center.latitude}, want ~ $lat');
    // longitude tightening by cos(lat)
    final lonTol = tolDeg / math.cos(lat * math.pi / 180.0);
    expect(_nearlyEqual(center.longitude, lon, lonTol), isTrue,
        reason: 'lon mismatch: got ${center.longitude}, want ~ $lon');
  });

  test('approxCellSizeMeters decreases with levels', () {
    final l2 = approxCellSizeMeters(2);
    final l3 = approxCellSizeMeters(3);
    final l4 = approxCellSizeMeters(4);
    expect(l2 > l3 && l3 > l4, isTrue);
  });

  test('formatting groups 10 chars as 4-4-2', () {
    final code = getDigiPin(0, 0, 10);
    final parts = code.split('-');
    expect(parts.length, 3);
    expect(parts[0].length, 4);
    expect(parts[1].length, 4);
    expect(parts[2].length, 2);
  });

  test('invalid chars cause error', () {
    expect(() => getLatLngFromDigiPin('@@@@'), throwsArgumentError);
  });
}
