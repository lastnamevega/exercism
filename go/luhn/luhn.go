package luhn

import (
	"strconv"
	"unicode"
)

func sanitize(s string) (sanitized string) {
	for _, character := range s {
		if unicode.IsDigit(character) {
			sanitized += string(character)
		} else if !unicode.IsSpace(character) {
			return ""
		}
	}

	return sanitized
}

func calculateSum(s string) (sum int, err error) {
	for i, character := range s {
		digit, err := strconv.ParseInt(string(character), 10, 64)

		if err != nil {
			return sum, err
		}

		if (len(s)-i)%2 == 0 {
			digit += digit
		}

		if digit > 9 {
			digit -= 9
		}

		sum += int(digit)
	}

	return sum, nil
}

func Valid(s string) bool {
	sanitized := sanitize(s)

	if len(sanitized) < 2 {
		return false
	}

	sum, err := calculateSum(sanitized)

	if err != nil {
		return false
	}

	return sum%10 == 0
}
