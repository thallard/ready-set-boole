package multiply

import "testing"

func TestMultiply(t *testing.T) {
	t.Run("Multiply 1 * 1", func(t *testing.T) {
		got := Multiply(1, 1)
		want := 1

		if got != want {
			t.Errorf("got %q want %q", got, want)
		}
	})

	t.Run("Multiply 0 * 0", func(t *testing.T) {
		got := Multiply(0, 0)
		want := 0

		if got != want {
			t.Errorf("got %q want %q", got, want)
		}
	})

	t.Run("Multiply 0 * 45", func(t *testing.T) {
		got := Multiply(0, 45)
		want := 0

		if got != want {
			t.Errorf("got %q want %q", got, want)
		}
	})

	t.Run("Multiply 3 * 2", func(t *testing.T) {
		got := Multiply(3, 2)
		want := 6

		if got != want {
			t.Errorf("got %q want %q", got, want)
		}
	})

	t.Run("Multiply 0 * 0", func(t *testing.T) {
		got := Multiply(0, 0)
		want := 0

		if got != want {
			t.Errorf("got %q want %q", got, want)
		}
	})
}
