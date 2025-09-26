package gda_test

import (
	"fmt"
	"testing"

	"github.com/sumitsharansatsangi/global_digital_address/packages/go/gda"
)

func ExampleGetDigiPin() {
	code, _ := gda.GetDigiPin(28.6139, 77.2090, 10) // New Delhi
	fmt.Println(len(code) >= 10)
	// Output: true
}

func TestRoundtripCode(t *testing.T) {
	code, err := gda.GetDigiPin(28.6139, 77.2090, 10)
	if err != nil {
		t.Fatalf("GetDigiPin: %v", err)
	}
	ll, err := gda.GetLatLngFromDigiPin(code)
	if err != nil {
		t.Fatalf("GetLatLngFromDigiPin: %v", err)
	}
	code2, err := gda.GetDigiPin(ll.Latitude, ll.Longitude, 10)
	if err != nil {
		t.Fatalf("GetDigiPin(center): %v", err)
	}
	if code2 != code {
		t.Fatalf("re-encoding center mismatch: got %q, want %q", code2, code)
	}
}

func TestApproxCellSizeMeters(t *testing.T) {
	sz, err := gda.ApproxCellSizeMeters(10)
	if err != nil {
		t.Fatalf("ApproxCellSizeMeters: %v", err)
	}
	if sz <= 0 {
		t.Fatalf("ApproxCellSizeMeters returned non-positive: %v", sz)
	}
}
