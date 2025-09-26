# global_digital_address

Square-cell **Web Mercator** encoder/decoder for globally unique grid codes (“DigiPins”) in Go.

- Deterministic 6×6 subdivision per symbol (base-36 alphabet from a fixed grid)
- Simple API: encode to a code, decode to cell center
- Default 10-symbol codes are grouped as `ABCD-EFGH-IJ`
- No dependencies

## Install

```bash
go get github.com/sumitsharansatsangi/global_digital_address
````

## Usage

```go
import gda "github.com/sumitsharansatsangi/global_digital_address"

// Encode (default 10 symbols)
pin, err := gda.GetDigiPin(28.6139, 77.2090)
// pin => e.g. "....-....-.."

// Decode
center, err := gda.GetLatLngFromDigiPin(pin)
// center.Latitude, center.Longitude

// Approximate cell size (meters) at N symbols
sz, _ := gda.ApproxCellSizeMeters(10)
```

## API

* `GetDigiPin(lat, lon float64, levels ...int) (string, error)`
* `GetLatLngFromDigiPin(code string) (LatLng, error)`
* `ApproxCellSizeMeters(levels int) (float64, error)`
* `var DIGIPIN_GRID [][]rune`
* `const MAX_LAT float64`

## Development

```bash
go test ./...
```

## License

MIT