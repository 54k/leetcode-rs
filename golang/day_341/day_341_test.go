package day341

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
