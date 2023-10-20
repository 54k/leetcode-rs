package day342

import (
	"unicode"
)

// https://leetcode.com/problems/basic-calculator-iii/description/
func calculate(s string) int {
	var solve func(string, *int) int
	solve = func(s string, i *int) int {
		stack := []int{}
		current := 0
		operator := '+'

		eval := func(op rune, x, y int) int {
			switch op {
			case '+':
				return x
			case '-':
				return -x
			case '*':
				return x * y
			default:
				return x / y
			}
		}

		for *i < len(s) {
			ch := rune(s[*i])
			if unicode.IsNumber(ch) {
				current = current*10 + int(ch-'0')
			} else if ch == '(' {
				*i = *i + 1
				current = solve(s, i)
			} else {
				if operator == '*' || operator == '/' {
					x := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					stack = append(stack, eval(operator, x, current))
				} else {
					stack = append(stack, eval(operator, current, 0))
				}

				if ch == ')' {
					break
				}

				operator = ch
				current = 0
			}

			*i = *i + 1
		}

		result := 0
		for _, n := range stack {
			result += n
		}
		return result
	}

	i := 0
	return solve(s+"@", &i)
}
