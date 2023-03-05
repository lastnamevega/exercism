package isogram

import (
	"strings"
	"unicode"
)

func isRepeatingLetter(character rune, remaining string) bool {
	isLetter := unicode.IsLetter(character)
	return isLetter && strings.ContainsRune(remaining, character)
}

func IsIsogram(word string) bool {
	lower := strings.ToLower(word)

	for i, character := range lower {
		if isRepeatingLetter(character, lower[i+1:]) {
			return false
		}
	}

	return true
}
