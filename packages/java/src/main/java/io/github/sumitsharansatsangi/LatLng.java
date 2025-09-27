package io.github.sumitsharansatsangi;

public final class LatLng {
    public final double latitude;
    public final double longitude;

    public LatLng(double latitude, double longitude) {
        this.latitude = latitude;
        this.longitude = longitude;
    }

    @Override
    public String toString() {
        return "LatLng{lat=" + latitude + ", lon=" + longitude + '}';
    }
}
