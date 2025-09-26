"use strict";
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// src/index.ts
var src_exports = {};
__export(src_exports, {
  DIGIPIN_GRID: () => DIGIPIN_GRID,
  MAX_LAT: () => MAX_LAT,
  approxCellSizeMeters: () => approxCellSizeMeters,
  getDigiPin: () => getDigiPin,
  getLatLngFromDigiPin: () => getLatLngFromDigiPin
});
module.exports = __toCommonJS(src_exports);
var DIGIPIN_GRID = [
  ["I", "A", "B", "C", "D", "E"],
  ["G", "H", "J", "K", "L", "M"],
  ["N", "P", "Q", "R", "S", "T"],
  ["U", "r", "W", "X", "Y", "Z"],
  ["a", "b", "9", "d", "V", "F"],
  ["2", "3", "4", "5", "6", "7"]
];
var R = 6378137;
var MAX_LAT = 85.05112878;
var PI = Math.PI;
var MERC_BOUNDS = {
  minX: -PI * R,
  maxX: PI * R,
  minY: -R * Math.log(Math.tan(PI / 4 + MAX_LAT * PI / 180 / 2)),
  maxY: R * Math.log(Math.tan(PI / 4 + MAX_LAT * PI / 180 / 2))
};
function clamp(v, lo, hi) {
  return Math.max(lo, Math.min(hi, v));
}
function normalizeLon(lon) {
  const x = ((lon + 180) % 360 + 360) % 360 - 180;
  return x === 180 ? -180 : x;
}
function lonToX(lonDeg) {
  return R * (normalizeLon(lonDeg) * PI / 180);
}
function xToLon(x) {
  return normalizeLon(x / R * 180 / PI);
}
function latToY(latDeg) {
  const \u03C6 = clamp(latDeg, -MAX_LAT, MAX_LAT) * PI / 180;
  return R * Math.log(Math.tan(PI / 4 + \u03C6 / 2));
}
function yToLat(y) {
  const \u03C6 = 2 * Math.atan(Math.exp(y / R)) - PI / 2;
  return \u03C6 * 180 / PI;
}
function getDigiPin(lat, lon, levels = 10) {
  if (!Number.isFinite(lat) || !Number.isFinite(lon)) throw new Error("lat/lon must be finite numbers");
  lat = clamp(lat, -MAX_LAT, MAX_LAT);
  lon = normalizeLon(lon);
  let x = lonToX(lon);
  let y = latToY(lat);
  const eps = 1e-9;
  let minX = MERC_BOUNDS.minX, maxX = MERC_BOUNDS.maxX;
  let minY = MERC_BOUNDS.minY, maxY = MERC_BOUNDS.maxY;
  x = Math.min(Math.max(x, minX + eps), maxX - eps);
  y = Math.min(Math.max(y, minY + eps), maxY - eps);
  let code = "";
  for (let level = 1; level <= levels; level++) {
    const xDiv = (maxX - minX) / 6;
    const yDiv = (maxY - minY) / 6;
    const rowRaw = 5 - Math.floor((y - minY) / yDiv);
    const colRaw = Math.floor((x - minX) / xDiv);
    const row = clamp(rowRaw, 0, 5);
    const col = clamp(colRaw, 0, 5);
    code += DIGIPIN_GRID[row][col];
    const newMaxY = minY + yDiv * (6 - row);
    const newMinY = minY + yDiv * (5 - row);
    minX = minX + xDiv * col;
    const newMaxX = minX + xDiv;
    minY = newMinY;
    maxY = newMaxY;
    maxX = newMaxX;
  }
  return code.length === 10 ? `${code.slice(0, 4)}-${code.slice(4, 8)}-${code.slice(8)}` : code.replace(/(.{4})/g, "$1-").replace(/-$/, "");
}
function getLatLngFromDigiPin(digiPin) {
  if (typeof digiPin !== "string") throw new Error("digiPin must be a string");
  const pin = digiPin.replace(/-/g, "");
  if (!pin || pin.length < 1) throw new Error("Invalid DIGIPIN");
  let minX = MERC_BOUNDS.minX, maxX = MERC_BOUNDS.maxX;
  let minY = MERC_BOUNDS.minY, maxY = MERC_BOUNDS.maxY;
  for (let i = 0; i < pin.length; i++) {
    const ch = pin[i];
    let ri = -1, ci = -1;
    outer: for (let r = 0; r < 6; r++) {
      for (let c = 0; c < 6; c++) {
        if (DIGIPIN_GRID[r][c] === ch) {
          ri = r;
          ci = c;
          break outer;
        }
      }
    }
    if (ri < 0) throw new Error(`Invalid character '${ch}' in DIGIPIN`);
    const xDiv = (maxX - minX) / 6;
    const yDiv = (maxY - minY) / 6;
    const y1 = maxY - yDiv * (ri + 1);
    const y2 = maxY - yDiv * ri;
    const x1 = minX + xDiv * ci;
    const x2 = x1 + xDiv;
    minY = y1;
    maxY = y2;
    minX = x1;
    maxX = x2;
  }
  const cx = (minX + maxX) / 2;
  const cy = (minY + maxY) / 2;
  return { latitude: yToLat(cy), longitude: xToLon(cx) };
}
function approxCellSizeMeters(levels) {
  const world = 2 * Math.PI * R;
  return world / Math.pow(6, levels);
}
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {
  DIGIPIN_GRID,
  MAX_LAT,
  approxCellSizeMeters,
  getDigiPin,
  getLatLngFromDigiPin
});
