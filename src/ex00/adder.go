package adder

func Adder(a int, b int) int {
	var carry int

	for b != 0 {
		carry = a & b
		a = a ^ b
		b = carry << 1
	}
	return a
}
