package io.github.sumitsharansatsangi;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class GlobalDigitalAddressTest {

    @Test
    void encodeDecode_roundtrip_isReasonablyAccurate() {
        double lat = 28.6139;   // New Delhi
        double lon = 77.2090;

        String digiPin = GlobalDigitalAddress.getDigiPin(lat, lon, 10);
        assertNotNull(digiPin, "DigiPin should not be null");
        assertTrue(digiPin.contains("-"), "DigiPin should contain dashes");
        assertEquals(12, digiPin.length(), "Expected 4-4-2 grouping");

        LatLng decoded = GlobalDigitalAddress.getLatLngFromDigiPin(digiPin);
        assertNotNull(decoded, "Decoded LatLng should not be null");

        // ~±0.05° tolerance (~5.5 km). Tighten if you want stricter checks.
        assertEquals(lat, decoded.latitude, 0.05, "Latitude should be close");
        assertEquals(lon, decoded.longitude, 0.05, "Longitude should be close");
    }

    @Test
    void approxCellSize_decreasesWithLevels() {
        double size10 = GlobalDigitalAddress.approxCellSizeMeters(10);
        double size5  = GlobalDigitalAddress.approxCellSizeMeters(5);

        assertTrue(size10 < 200, "Level 10 cell size should be small");
        assertTrue(size5 > size10, "Coarser levels should be larger");
    }

    @Test
    void invalidInputs_throwHelpfulExceptions() {
        assertThrows(IllegalArgumentException.class,
                () -> GlobalDigitalAddress.getDigiPin(Double.NaN, 77.0, 10));

        assertThrows(IllegalArgumentException.class,
                () -> GlobalDigitalAddress.getLatLngFromDigiPin("INVALID$PIN"));

        assertThrows(IllegalArgumentException.class,
                () -> GlobalDigitalAddress.approxCellSizeMeters(0));
    }
}
