package raindrops

import "fmt"

func isDivisibleBy(number int, divisor int) bool {
	return number%divisor == 0
}

func Convert(number int) (converted string) {
	if isDivisibleBy(number, 3) {
		converted += "Pling"
	}

	if isDivisibleBy(number, 5) {
		converted += "Plang"
	}

	if isDivisibleBy(number, 7) {
		converted += "Plong"
	}

	if len(converted) == 0 {
		converted = fmt.Sprint(number)
	}

	return converted
}
