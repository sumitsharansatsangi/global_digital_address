from __future__ import annotations
import math
from typing import List, Tuple

DIGIPIN_GRID: List[List[str]] = [
    ['I', 'A', 'B', 'C', 'D', 'E'],
    ['G', 'H', 'J', 'K', 'L', 'M'],
    ['N', 'P', 'Q', 'R', 'S', 'T'],
    ['U', 'r', 'W', 'X', 'Y', 'Z'],
    ['a', 'b', '9', 'd', 'V', 'F'],
    ['2', '3', '4', '5', '6', '7'],
]

R = 6378137.0  # meters
MAX_LAT = 85.05112878  # Mercator limit in degrees
PI = math.pi

MERC_BOUNDS = {
    "minX": -PI * R,
    "maxX":  PI * R,
    "minY": -R * math.log(math.tan(PI / 4 + (MAX_LAT * PI / 180) / 2)),
    "maxY":  R * math.log(math.tan(PI / 4 + (MAX_LAT * PI / 180) / 2)),
}

def _clamp(v: float, lo: float, hi: float) -> float:
    return max(lo, min(hi, v))

def _normalize_lon(lon: float) -> float:
    x = ((lon + 180.0) % 360.0 + 360.0) % 360.0 - 180.0
    return -180.0 if x == 180.0 else x

def _lon_to_x(lon_deg: float) -> float:
    return R * (_normalize_lon(lon_deg) * PI / 180.0)

def _x_to_lon(x: float) -> float:
    return _normalize_lon((x / R) * 180.0 / PI)

def _lat_to_y(lat_deg: float) -> float:
    phi = _clamp(lat_deg, -MAX_LAT, MAX_LAT) * PI / 180.0
    return R * math.log(math.tan(PI / 4.0 + phi / 2.0))

def _y_to_lat(y: float) -> float:
    phi = 2.0 * math.atan(math.exp(y / R)) - PI / 2.0
    return phi * 180.0 / PI

def get_digipin(lat: float, lon: float, levels: int = 10) -> str:
    if not (math.isfinite(lat) and math.isfinite(lon)):
        raise ValueError("lat/lon must be finite numbers")

    lat = _clamp(lat, -MAX_LAT, MAX_LAT)
    lon = _normalize_lon(lon)

    x = _lon_to_x(lon)
    y = _lat_to_y(lat)

    eps = 1e-9
    minX = MERC_BOUNDS["minX"]
    maxX = MERC_BOUNDS["maxX"]
    minY = MERC_BOUNDS["minY"]
    maxY = MERC_BOUNDS["maxY"]

    x = min(max(x, minX + eps), maxX - eps)
    y = min(max(y, minY + eps), maxY - eps)

    code_chars: List[str] = []
    for _level in range(1, levels + 1):
        xDiv = (maxX - minX) / 6.0
        yDiv = (maxY - minY) / 6.0

        row_raw = 5 - int(math.floor((y - minY) / yDiv))
        col_raw = int(math.floor((x - minX) / xDiv))
        row = int(_clamp(row_raw, 0, 5))
        col = int(_clamp(col_raw, 0, 5))

        code_chars.append(DIGIPIN_GRID[row][col])

        newMaxY = minY + yDiv * (6 - row)
        newMinY = minY + yDiv * (5 - row)
        minX = minX + xDiv * col
        newMaxX = minX + xDiv

        minY, maxY = newMinY, newMaxY
        maxX = newMaxX

    code = "".join(code_chars)

    if len(code) == 10:
        return f"{code[:4]}-{code[4:8]}-{code[8:]}"
    return "-".join(code[i:i+4] for i in range(0, len(code), 4))

def get_latlng_from_digipin(digipin: str) -> Tuple[float, float]:
    if not isinstance(digipin, str):
        raise ValueError("digiPin must be a string")
    pin = digipin.replace("-", "")
    if not pin:
        raise ValueError("Invalid DIGIPIN")

    minX = MERC_BOUNDS["minX"]
    maxX = MERC_BOUNDS["maxX"]
    minY = MERC_BOUNDS["minY"]
    maxY = MERC_BOUNDS["maxY"]

    for ch in pin:
        ri = ci = -1
        for r in range(6):
            for c in range(6):
                if DIGIPIN_GRID[r][c] == ch:
                    ri, ci = r, c
                    break
            if ri >= 0:
                break
        if ri < 0:
            raise ValueError(f"Invalid character '{ch}' in DIGIPIN")

        xDiv = (maxX - minX) / 6.0
        yDiv = (maxY - minY) / 6.0

        y1 = maxY - yDiv * (ri + 1)
        y2 = maxY - yDiv * ri
        x1 = minX + xDiv * ci
        x2 = x1 + xDiv

        minY, maxY = y1, y2
        minX, maxX = x1, x2

    cx = (minX + maxX) / 2.0
    cy = (minY + maxY) / 2.0

    return (_y_to_lat(cy), _x_to_lon(cx))

def approx_cell_size_meters(levels: int) -> float:
    world = 2.0 * math.pi * R
    return world / (6.0 ** levels)
