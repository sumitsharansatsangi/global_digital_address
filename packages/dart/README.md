# `README.md`

# global_digital_address

Square-cell Web Mercator encoder/decoder for globally unique grid codes (“DigiPin”).

## Features

- Encode latitude/longitude to a grid code with configurable precision (levels).
- Decode a code back to the center coordinate of its cell.
- Works on the spherical Web Mercator domain (±85.05112878°).
- Deterministic formatting (10 chars → `4-4-2` grouping).
- Utility to estimate cell size in meters for a given code length.

## Install

```sh
dart pub add global_digital_address
````

## Usage

```dart
import 'package:global_digital_address/global_digital_address.dart';

void main() {
  final pin = getDigiPin(28.6139, 77.2090); // New Delhi
  final center = getLatLngFromDigiPin(pin);

  print('DigiPin: $pin');
  print('Center: ${center.latitude}, ${center.longitude}');
  print('~cell size @10: ${approxCellSizeMeters(10)} m');
}
```

## API

* `String getDigiPin(double lat, double lon, [int levels = 10])`
* `LatLng getLatLngFromDigiPin(String digiPin)`
* `double approxCellSizeMeters(int levels)`
* `const DIGIPIN_GRID`, `const MAX_LAT`

## Notes

* Longitudes normalized to `[-180, 180)`. Latitudes clamped to `±85.05112878°`.
* The algorithm subdivides the global Mercator extent into a 6×6 grid at each level.

## License

MIT
