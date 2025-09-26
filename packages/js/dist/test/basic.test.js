import { describe, it, expect } from "vitest";
import { getDigiPin, getLatLngFromDigiPin, approxCellSizeMeters } from "../src";
describe("global_digital_address", () => {
    it("roundtrips a known coordinate", () => {
        const lat = 28.6139, lon = 77.2090; // New Delhi (approx)
        const code = getDigiPin(lat, lon, 10);
        const { latitude, longitude } = getLatLngFromDigiPin(code);
        expect(Math.abs(latitude - lat)).toBeLessThan(0.001); // ~100 m tolerance (center of cell)
        expect(Math.abs(longitude - lon)).toBeLessThan(0.001);
    });
    it("cell size shrinks with level", () => {
        const s1 = approxCellSizeMeters(6);
        const s2 = approxCellSizeMeters(7);
        expect(s2).toBeLessThan(s1);
    });
});
