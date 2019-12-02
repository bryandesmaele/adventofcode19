package adventofcode1901

import "testing"

func TestCalc(t *testing.T) {
	want := 2
	mass := 12
	if got := Calc(mass); got != want {
		t.Errorf("Calc(%q) = %q, want %q", mass, got, want)
	}

	want = 2
	mass = 14
	if got := Calc(mass); got != want {
		t.Errorf("Calc(%q) = %q, want %q", mass, got, want)
	}

	want = 33583
	mass = 100756
	if got := Calc(mass); got != want {
		t.Errorf("Calc(%q) = %q, want %q", mass, got, want)
	}

	want = 654
	mass = 1969
	if got := Calc(mass); got != want {
		t.Errorf("Calc(%q) = %q, want %q", mass, got, want)
	}
}
