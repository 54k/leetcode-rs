package day300

// https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/description/
func minSwaps(s string) int {
	bal := 0
	for _, v := range s {
		if v == '[' {
			bal++
		} else if bal > 0 {
			bal--
		}
	}
	return (bal + 1) / 2
}
