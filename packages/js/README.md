## `README.md`

````md
# global_digital_address

Square-cell Web Mercator **global digital address** encoder/decoder (DIGIPIN-like).  
Each character subdivides the world into a 6×6 grid ⇒ **square cells** in projected meters.

- **Global, unique, gap-free** coverage from **−85.05112878° to +85.05112878°** latitude
- **Square cells** at every level (Web Mercator)
- Tiny, dependency-free, TypeScript types included
- Works in Node and browsers

## Install

```bash
npm i global_digital_address
# or
yarn add global_digital_address
# or
pnpm add global_digital_address
````

## Usage

```ts
import { getDigiPin, getLatLngFromDigiPin } from "global_digital_address";

const code = getDigiPin(28.6139, 77.2090, 10); // "ABCD-EFGH-IJ" (example)
const { latitude, longitude } = getLatLngFromDigiPin(code);
```

### API

* `getDigiPin(lat, lon, levels = 10): string`

  * Returns grouped code (4-4-2 for length 10; otherwise groups of 4).
* `getLatLngFromDigiPin(code: string): { latitude, longitude }`

  * Returns center of the decoded square cell.
* `approxCellSizeMeters(levels: number): number`

  * Returns approximate cell width/height (meters at equator).

### Grid & Coverage

* Projection: **Web Mercator**
* Max usable latitude: **±85.05112878°**
* Each additional character increases resolution by ×6 in both axes.
