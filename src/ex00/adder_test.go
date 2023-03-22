package adder

import "testing"

func TestAdder(t *testing.T) {
	t.Run("Add 1 + 1", func(t *testing.T) {
		got := Adder(1, 1)
		want := 2

		if got != want {
			t.Errorf("got %q want %q", got, want)
		}
	})

	t.Run("Add 0 + 0", func(t *testing.T) {
		got := Adder(0, 0)
		want := 0

		if got != want {
			t.Errorf("got %q want %q", got, want)
		}
	})

	t.Run("Add 0 + 45", func(t *testing.T) {
		got := Adder(0, 45)
		want := 45

		if got != want {
			t.Errorf("got %q want %q", got, want)
		}
	})

	t.Run("Add 3 + 2", func(t *testing.T) {
		got := Adder(3, 2)
		want := 5

		if got != want {
			t.Errorf("got %q want %q", got, want)
		}
	})
}
