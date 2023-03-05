package acronym

import (
	"regexp"
	"strings"
)

func Abbreviate(s string) (initialism string) {
	re := regexp.MustCompile("[^a-zA-Z']+")
	split := re.Split(s, -1)

	for _, word := range split {
		initialism += string(word[0])
	}

	return strings.ToUpper(initialism)
}
