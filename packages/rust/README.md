## `README.md`


# global_digital_address

Square-cell Web Mercator encoder/decoder for globally unique grid codes (6×6 subdivision per level).

## Features
- Web Mercator (EPSG:3857) math with `MAX_LAT = 85.05112878°`
- Deterministic 6×6 symbol grid per level
- Grouping: `AAAA-BBBB-CC` for 10-char codes; otherwise groups of 4
- Reverse decode to cell center
- Approximate cell size by code length

## Install
```toml
# Cargo.toml
[dependencies]
global_digital_address = "1.0"
````

## Usage

```rust
use global_digital_address::{get_digi_pin, get_lat_lng_from_digipin};

let code = get_digi_pin(28.6139, 77.2090, 10).unwrap();
let center = get_lat_lng_from_digipin(&code).unwrap();
println!("{code} -> {}, {}", center.latitude, center.longitude);
```

## CLI example

```
cargo run --example encode_decode
```

## License

MIT © Sumit Kumar