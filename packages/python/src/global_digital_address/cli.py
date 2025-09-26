import argparse
from .core import get_digipin, get_latlng_from_digipin, approx_cell_size_meters

def main() -> None:
    parser = argparse.ArgumentParser(prog="gda", description="Global Digital Address (DIGIPIN)")
    sub = parser.add_subparsers(dest="cmd", required=True)

    enc = sub.add_parser("encode", help="Encode lat lon to DIGIPIN")
    enc.add_argument("lat", type=float)
    enc.add_argument("lon", type=float)
    enc.add_argument("--levels", type=int, default=10, help="code length (symbols)")

    dec = sub.add_parser("decode", help="Decode DIGIPIN to lat lon center")
    dec.add_argument("code", type=str)

    size = sub.add_parser("size", help="Approximate cell size at levels")
    size.add_argument("levels", type=int)

    args = parser.parse_args()
    if args.cmd == "encode":
        print(get_digipin(args.lat, args.lon, args.levels))
    elif args.cmd == "decode":
        lat, lon = get_latlng_from_digipin(args.code)
        print(f"{lat:.8f} {lon:.8f}")
    elif args.cmd == "size":
        print(f"{approx_cell_size_meters(args.levels):.6f}")
