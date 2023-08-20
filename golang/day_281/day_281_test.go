package day281

// https://leetcode.com/problems/maximize-the-confusion-of-an-exam/description/
func maxConsecutiveAnswers1(answerKey string, k int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	isValid := func(size int) bool {
		fCount := 0
		tCount := 0

		for i := 0; i < len(answerKey); i++ {
			if answerKey[i] == 'T' {
				tCount++
			} else {
				fCount++
			}

			if i >= size {
				if answerKey[i-size] == 'T' {
					tCount--
				} else {
					fCount--
				}
			}

			if i+1 >= size {
				if min(fCount, tCount) <= k {
					return true
				}
			}
		}
		return false
	}

	lo, hi := k, len(answerKey)
	for lo < hi {
		mid := (lo + hi + 1) / 2
		if isValid(mid) {
			lo = mid
		} else {
			hi = mid - 1
		}
	}
	return lo
}

func maxConsecutiveAnswers2(answerKey string, k int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	fc, tc := 0, 0
	lo := 0
	ans := 0
	for hi := 0; hi < len(answerKey); hi++ {
		if answerKey[hi] == 'T' {
			tc++
		} else {
			fc++
		}
		for min(fc, tc) > k {
			if answerKey[lo] == 'T' {
				tc--
			} else {
				fc--
			}
			lo++
		}
		ans = max(ans, hi-lo+1)
	}
	return ans
}

func maxConsecutiveAnswers3(answerKey string, k int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	fc, tc, ans := 0, 0, 0
	left := 0
	for right := 0; right < len(answerKey); right++ {
		if answerKey[right] == 'T' {
			tc++
		} else {
			fc++
		}
		minor := min(tc, fc)
		if minor <= k {
			ans++
		} else {
			if answerKey[left] == 'T' {
				tc--
			} else {
				fc--
			}
			left++
		}
	}
	return ans
}

// https://leetcode.com/problems/check-if-a-string-is-an-acronym-of-words/
func isAcronym(words []string, s string) bool {
	acr := ""
	for _, w := range words {
		acr += string(w[0])
	}
	return acr == s
}

// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/description/
func minAddToMakeValid(s string) int {
	bal := 0
	ans := 0
	for _, b := range s {
		if b == '(' {
			bal += 1
		} else if b == ')' {
			bal -= 1
		}
		if bal == -1 {
			ans++
			bal++
		}
	}
	return ans + bal
}
