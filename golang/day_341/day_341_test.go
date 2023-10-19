package day341

import "unicode"

// https://leetcode.com/problems/backspace-string-compare/description
func backspaceCompare(s string, t string) bool {
	i, j := len(s)-1, len(t)-1

	for i >= 0 || j >= 0 {
		bs := 0
		for i >= 0 {
			if s[i] == '#' {
				bs++
				i--
			} else if bs > 0 {
				bs--
				i--
			} else {
				break
			}
		}

		bt := 0
		for j >= 0 {
			if t[j] == '#' {
				bt++
				j--
			} else if bt > 0 {
				bt--
				j--
			} else {
				break
			}
		}

		if i >= 0 && j >= 0 && s[i] != t[j] {
			return false
		}

		i--
		j--
	}
	return i == j
}

// https://leetcode.com/problems/basic-calculator/
func calculate(s string) int {
	result := 0
	stack := []int{}
	operator := 1
	curr := 0

	for _, ch := range s {
		if unicode.IsDigit(ch) {
			curr = curr*10 + int(ch-'0')
		} else if ch == '+' || ch == '-' {
			result += curr * operator
			curr = 0
			operator = 1
			if ch == '-' {
				operator = -1
			}
		} else if ch == '(' {
			stack = append(stack, result)
			stack = append(stack, operator)

			result = 0
			operator = 1
			curr = 0
		} else if ch == ')' {
			result += curr * operator

			result *= stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			result += stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			curr = 0
			operator = 1
		}
	}

	return result + (curr * operator)
}
