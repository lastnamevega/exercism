package grains

import "errors"

func Square(n int) (uint64, error) {
	if n < 1 || n > 64 {
		return 0, errors.New("Input must be between 1 and 64, inclusive")
	}

	return 1 << (n - 1), nil
}

func Total() uint64 {
	return 18446744073709551615
}
