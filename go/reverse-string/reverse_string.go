package reverse

func Reverse(s string) (reversed string) {
	for _, character := range s {
		reversed = string(character) + reversed
	}

	return reversed
}
