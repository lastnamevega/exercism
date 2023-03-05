package scrabble

import "strings"

func letterScore(character rune) int {
	switch character {
	case 'a', 'e', 'i', 'o', 'u', 'l', 'n', 'r', 's', 't':
		return 1
	case 'd', 'g':
		return 2
	case 'b', 'c', 'm', 'p':
		return 3
	case 'f', 'h', 'v', 'w', 'y':
		return 4
	case 'k':
		return 5
	case 'j', 'x':
		return 8
	case 'q', 'z':
		return 10
	default:
		return 0
	}
}

func Score(play string) (score int) {
	lower := strings.ToLower(play)

	for _, character := range lower {
		score += letterScore(character)
	}

	return score
}
