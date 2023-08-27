package day288

// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/description/
func maxDepth(s string) int {
	ans := 0
	bal := 0
	for _, ch := range s {
		if ch == '(' {
			bal++
		} else if ch == ')' {
			if ans < bal {
				ans = bal
			}
			bal--
		}
	}
	return ans
}
