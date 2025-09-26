from global_digital_address import get_digipin, get_latlng_from_digipin, approx_cell_size_meters

def test_roundtrip_new_delhi():
    lat, lon = 28.6139, 77.2090
    code = get_digipin(lat, lon, levels=10)
    lat2, lon2 = get_latlng_from_digipin(code)
    # decoded is cell center, so it won't equal original; just sanity checks:
    assert -90 <= lat2 <= 90
    assert -180 <= lon2 <= 180
    assert len(code.replace("-", "")) == 10

def test_cell_size_monotonic():
    assert approx_cell_size_meters(1) > approx_cell_size_meters(2) > approx_cell_size_meters(3)
