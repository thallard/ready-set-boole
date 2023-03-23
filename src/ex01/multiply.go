package multiply

import adder "src/src/ex00"

func Multiply(a int, b int) int {
	var result int

	for b != 0 {
		if b&1 == 1 {
			result = adder.Adder(result, a)
		}
		a <<= 1
		b >>= 1
	}

	return result
}
