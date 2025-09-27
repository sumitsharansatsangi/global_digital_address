package io.github.sumitsharansatsangi;

/**
 * global_digital_address
 * Square-cell Web Mercator encoder/decoder for globally unique grid codes.
 */
public final class GlobalDigitalAddress {

    private GlobalDigitalAddress() {}

    public static final String[][] DIGIPIN_GRID = {
            {"I", "A", "B", "C", "D", "E"},
            {"G", "H", "J", "K", "L", "M"},
            {"N", "P", "Q", "R", "S", "T"},
            {"U", "r", "W", "X", "Y", "Z"},
            {"a", "b", "9", "d", "V", "F"},
            {"2", "3", "4", "5", "6", "7"},
    };

    private static final double R = 6_378_137.0;           
    public static final double MAX_LAT = 85.05112878;      
    private static final double PI = Math.PI;

    private static final double MIN_X = -PI * R;
    private static final double MAX_X =  PI * R;
    private static final double MIN_Y;
    private static final double MAX_Y;

    static {
        double maxLatRad = Math.toRadians(MAX_LAT);
        MIN_Y = -R * Math.log(Math.tan(PI / 4 + maxLatRad / 2));
        MAX_Y =  R * Math.log(Math.tan(PI / 4 + maxLatRad / 2));
    }

    private static double clamp(double v, double lo, double hi) {
        return Math.max(lo, Math.min(hi, v));
    }

    private static double normalizeLon(double lonDeg) {
        double x = ((lonDeg + 180.0) % 360.0 + 360.0) % 360.0 - 180.0;
        return (x == 180.0) ? -180.0 : x;
    }

    private static double lonToX(double lonDeg) { return R * Math.toRadians(normalizeLon(lonDeg)); }
    private static double xToLon(double x) { return normalizeLon(Math.toDegrees(x / R)); }
    private static double latToY(double latDeg) {
        double phi = Math.toRadians(clamp(latDeg, -MAX_LAT, MAX_LAT));
        return R * Math.log(Math.tan(PI / 4.0 + phi / 2.0));
    }
    private static double yToLat(double y) {
        double phi = 2.0 * Math.atan(Math.exp(y / R)) - PI / 2.0;
        return Math.toDegrees(phi);
    }

    public static String getDigiPin(double lat, double lon, int levels) {
        if (!Double.isFinite(lat) || !Double.isFinite(lon)) {
            throw new IllegalArgumentException("lat/lon must be finite numbers");
        }
        if (levels <= 0) levels = 10;

        lat = clamp(lat, -MAX_LAT, MAX_LAT);
        lon = normalizeLon(lon);

        double x = lonToX(lon);
        double y = latToY(lat);

        double eps = 1e-9;
        double minX = MIN_X, maxX = MAX_X;
        double minY = MIN_Y, maxY = MAX_Y;

        x = Math.min(Math.max(x, minX + eps), maxX - eps);
        y = Math.min(Math.max(y, minY + eps), maxY - eps);

        StringBuilder code = new StringBuilder(levels);

        for (int level = 1; level <= levels; level++) {
            double xDiv = (maxX - minX) / 6.0;
            double yDiv = (maxY - minY) / 6.0;

            int rowRaw = 5 - (int) Math.floor((y - minY) / yDiv);
            int colRaw = (int) Math.floor((x - minX) / xDiv);
            int row = (int) clamp(rowRaw, 0, 5);
            int col = (int) clamp(colRaw, 0, 5);

            code.append(DIGIPIN_GRID[row][col]);

            double newMaxY = minY + yDiv * (6 - row);
            double newMinY = minY + yDiv * (5 - row);
            minX = minX + xDiv * col;
            double newMaxX = minX + xDiv;

            minY = newMinY; maxY = newMaxY;
            maxX = newMaxX;
        }

        return groupCode(code.toString());
    }

    private static String groupCode(String raw) {
        if (raw.length() == 10) {
            return raw.substring(0, 4) + "-" + raw.substring(4, 8) + "-" + raw.substring(8);
        }
        StringBuilder sb = new StringBuilder(raw.length() + raw.length() / 4);
        for (int i = 0; i < raw.length(); i++) {
            if (i > 0 && i % 4 == 0) sb.append('-');
            sb.append(raw.charAt(i));
        }
        return sb.toString();
    }

    public static LatLng getLatLngFromDigiPin(String digiPin) {
        if (digiPin == null) throw new IllegalArgumentException("digiPin must be a string");
        String pin = digiPin.replace("-", "");
        if (pin.isEmpty()) throw new IllegalArgumentException("Invalid DIGIPIN");

        double minX = MIN_X, maxX = MAX_X;
        double minY = MIN_Y, maxY = MAX_Y;

        for (int i = 0; i < pin.length(); i++) {
            String ch = String.valueOf(pin.charAt(i));
            int ri = -1, ci = -1;

            outer:
            for (int r = 0; r < 6; r++) {
                for (int c = 0; c < 6; c++) {
                    if (DIGIPIN_GRID[r][c].equals(ch)) {
                        ri = r; ci = c; break outer;
                    }
                }
            }
            if (ri < 0) throw new IllegalArgumentException("Invalid character '" + ch + "' in DIGIPIN");

            double xDiv = (maxX - minX) / 6.0;
            double yDiv = (maxY - minY) / 6.0;

            double y1 = maxY - yDiv * (ri + 1);
            double y2 = maxY - yDiv * ri;
            double x1 = minX + xDiv * ci;
            double x2 = x1 + xDiv;

            minY = y1; maxY = y2;
            minX = x1; maxX = x2;
        }

        double cx = (minX + maxX) / 2.0;
        double cy = (minY + maxY) / 2.0;

        return new LatLng(yToLat(cy), xToLon(cx));
    }

    public static double approxCellSizeMeters(int levels) {
        if (levels <= 0) throw new IllegalArgumentException("levels must be >= 1");
        double world = 2.0 * Math.PI * R;
        return world / Math.pow(6.0, levels);
    }
}
