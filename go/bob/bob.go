package bob

import (
	"strings"
	"unicode"
)

func IsYelling(remark string) bool {
	isUpper := remark == strings.ToUpper(remark)
	containsLetter := strings.IndexFunc(remark, unicode.IsLetter) > -1

	return isUpper && containsLetter
}

func IsQuestion(remark string) bool {
	return strings.HasSuffix(remark, "?")
}

func IsYellingQuestion(remark string) bool {
	return IsYelling(remark) && IsQuestion(remark)
}

func IsSilence(remark string) bool {
	return remark == ""
}

func Hey(remark string) string {
	trimmed := strings.TrimSpace(remark)

	switch {
	case IsYellingQuestion(trimmed):
		return "Calm down, I know what I'm doing!"
	case IsYelling(trimmed):
		return "Whoa, chill out!"
	case IsQuestion(trimmed):
		return "Sure."
	case IsSilence(trimmed):
		return "Fine. Be that way!"
	default:
		return "Whatever."
	}
}
