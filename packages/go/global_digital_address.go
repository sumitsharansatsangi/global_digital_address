// Package global_digital_address 
// provides a square-cell Web Mercator encoder/decoder for globally
// unique grid codes.
// API:
//   - GetDigiPin(lat, lon, levels...?)
//   - GetLatLngFromDigiPin(code)
//   - ApproxCellSizeMeters(levels)
//   - constants/vars: DIGIPIN_GRID, MAX_LAT
package global_digital_address

import (
	"errors"
	"math"
	"strings"
	"unicode/utf8"
)

// DIGIPIN_GRID is the 6×6 symbol grid used at every level.
var DIGIPIN_GRID = [][]rune{
	{'I', 'A', 'B', 'C', 'D', 'E'},
	{'G', 'H', 'J', 'K', 'L', 'M'},
	{'N', 'P', 'Q', 'R', 'S', 'T'},
	{'U', 'r', 'W', 'X', 'Y', 'Z'},
	{'a', 'b', '9', 'd', 'V', 'F'},
	{'2', '3', '4', '5', '6', '7'},
}

const (
	// R is the Web Mercator sphere radius (meters).
	R = 6378137.0
	// MAX_LAT is the Web Mercator latitude limit in degrees.
	MAX_LAT = 85.05112878
	PI      = math.Pi
)

var mercBounds = struct {
	minX, maxX, minY, maxY float64
}{
	minX: -PI * R,
	maxX: PI * R,
	minY: -R * math.Log(math.Tan(PI/4+(MAX_LAT*PI/180)/2)),
	maxY: R * math.Log(math.Tan(PI/4+(MAX_LAT*PI/180)/2)),
}

func clamp(v, lo, hi float64) float64 {
	return math.Max(lo, math.Min(hi, v))
}

func normalizeLon(lon float64) float64 {
	x := math.Mod(lon+180.0, 360.0)
	if x < 0 {
		x += 360.0
	}
	x -= 180.0
	if math.Abs(x-180.0) < 1e-12 {
		return -180.0
	}
	return x
}

func lonToX(lonDeg float64) float64 { return R * (normalizeLon(lonDeg) * PI / 180.0) }
func xToLon(x float64) float64      { return normalizeLon((x / R) * 180.0 / PI) }

func latToY(latDeg float64) float64 {
	phi := clamp(latDeg, -MAX_LAT, MAX_LAT) * PI / 180.0
	return R * math.Log(math.Tan(PI/4.0+phi/2.0))
}
func yToLat(y float64) float64 {
	phi := 2.0*math.Atan(math.Exp(y/R)) - PI/2.0
	return phi * 180.0 / PI
}

// GetDigiPin encodes (lat, lon) to a square-cell global code.
// levels is optional (defaults to 10 if omitted).
// Returns grouped string like "ABCD-EFGH-IJ".
func GetDigiPin(lat, lon float64, levels ...int) (string, error) {
	if math.IsNaN(lat) || math.IsInf(lat, 0) || math.IsNaN(lon) || math.IsInf(lon, 0) {
		return "", errors.New("lat/lon must be finite numbers")
	}
	lvl := 10
	if len(levels) > 0 {
		lvl = levels[0]
	}
	if lvl <= 0 {
		return "", errors.New("levels must be positive")
	}

	lat = clamp(lat, -MAX_LAT, MAX_LAT)
	lon = normalizeLon(lon)

	// to Mercator (meters)
	x := lonToX(lon)
	y := latToY(lat)

	const eps = 1e-9
	minX, maxX := mercBounds.minX, mercBounds.maxX
	minY, maxY := mercBounds.minY, mercBounds.maxY

	if x < minX+eps {
		x = minX + eps
	} else if x > maxX-eps {
		x = maxX - eps
	}
	if y < minY+eps {
		y = minY + eps
	} else if y > maxY-eps {
		y = maxY - eps
	}

	var b strings.Builder
	for level := 1; level <= lvl; level++ {
		xDiv := (maxX - minX) / 6.0
		yDiv := (maxY - minY) / 6.0

		rowRaw := 5.0 - math.Floor((y-minY)/yDiv) // top row = 0
		colRaw := math.Floor((x - minX) / xDiv)
		row := int(clamp(rowRaw, 0, 5))
		col := int(clamp(colRaw, 0, 5))

		b.WriteRune(DIGIPIN_GRID[row][col])

		newMaxY := minY + yDiv*float64(6-row)
		newMinY := minY + yDiv*float64(5-row)
		minX = minX + xDiv*float64(col)
		newMaxX := minX + xDiv

		minY, maxY = newMinY, newMaxY
		maxX = newMaxX
	}

	code := b.String()
	if utf8.RuneCountInString(code) == 10 {
		return code[0:4] + "-" + code[4:8] + "-" + code[8:], nil
	}

	var grouped strings.Builder
	for i := 0; i < len(code); i += 4 {
		end := i + 4
		if end > len(code) {
			end = len(code)
		}
		grouped.WriteString(code[i:end])
		if end != len(code) {
			grouped.WriteByte('-')
		}
	}
	return grouped.String(), nil
}

// LatLng holds a latitude/longitude pair (degrees, WGS-84).
type LatLng struct {
	Latitude  float64
	Longitude float64
}

// GetLatLngFromDigiPin decodes a code back to the center lat/lon of its cell.
func GetLatLngFromDigiPin(digiPin string) (LatLng, error) {
	if digiPin == "" {
		return LatLng{}, errors.New("digiPin must be a non-empty string")
	}
	pin := strings.ReplaceAll(digiPin, "-", "")
	if pin == "" {
		return LatLng{}, errors.New("invalid DIGIPIN")
	}

	minX, maxX := mercBounds.minX, mercBounds.maxX
	minY, maxY := mercBounds.minY, mercBounds.maxY

	for _, ch := range pin {
		ri, ci := -1, -1
	outer:
		for r := 0; r < 6; r++ {
			for c := 0; c < 6; c++ {
				if DIGIPIN_GRID[r][c] == ch {
					ri, ci = r, c
					break outer
				}
			}
		}
		if ri < 0 {
			return LatLng{}, errors.New("invalid character '" + string(ch) + "' in DIGIPIN")
		}

		xDiv := (maxX - minX) / 6.0
		yDiv := (maxY - minY) / 6.0

		y1 := maxY - yDiv*float64(ri+1)
		y2 := maxY - yDiv*float64(ri)
		x1 := minX + xDiv*float64(ci)
		x2 := x1 + xDiv

		minY, maxY = y1, y2
		minX, maxX = x1, x2
	}

	cx := (minX + maxX) / 2.0
	cy := (minY + maxY) / 2.0

	return LatLng{
		Latitude:  yToLat(cy),
		Longitude: xToLon(cx),
	}, nil
}

// ApproxCellSizeMeters returns the approximate cell size (meters) for a given code length.
func ApproxCellSizeMeters(levels int) (float64, error) {
	if levels <= 0 {
		return 0, errors.New("levels must be positive")
	}
	world := 2.0 * math.Pi * R
	return world / math.Pow(6.0, float64(levels)), nil
}
