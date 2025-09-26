import 'package:global_digital_address/global_digital_address.dart';

void main() {
  final pin = getDigiPin(28.6139, 77.2090); // New Delhi
  final center = getLatLngFromDigiPin(pin);

  print('DigiPin: $pin');
  print('Center: ${center.latitude}, ${center.longitude}');
  print('~cell size @10: ${approxCellSizeMeters(10)} m');
}
