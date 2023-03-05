package hamming

import "errors"

func Distance(a, b string) (distance int, err error) {
	aRunes := []rune(a)
	bRunes := []rune(b)

	if len(aRunes) != len(bRunes) {
		return distance, errors.New("Strings must be of equal length")
	}

	for i, r := range aRunes {
		if r != bRunes[i] {
			distance++
		}
	}

	return
}
