package clock

import "fmt"

const dayMinutes = 24 * 60

type Clock struct {
	minutes int
}

func calculate(m int) int {
	for {
		if m > -1 {
			return m % dayMinutes
		}

		m += dayMinutes
	}
}

func New(h int, m int) Clock {
	return Clock{calculate(h*60 + m)}
}

func (c Clock) String() string {
	return fmt.Sprintf("%02d:%02d", c.minutes/60, c.minutes%60)
}

func (c Clock) Add(m int) Clock {
	c.minutes = calculate(c.minutes + m)

	return c
}

func (c Clock) Subtract(m int) Clock {
	return c.Add(-m)
}
