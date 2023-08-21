package day281

import (
	"sort"
	"strconv"
)

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

// https://leetcode.com/problems/find-right-interval/description/
func findRightInterval(intervals [][]int) []int {
	ktos := func(key []int) string {
		res := ""
		for _, e := range key {
			res += strconv.Itoa(e) + ","
		}
		return res
	}

	res := make([]int, len(intervals))

	hash := map[string]int{}
	for i, interval := range intervals {
		hash[ktos(interval)] = i
	}

	sort.Slice(intervals, func(a, b int) bool {
		return intervals[a][0] < intervals[b][0]
	})

	var binSearch func(int, int, int) []int
	binSearch = func(target int, start int, end int) []int {
		if start >= end {
			if intervals[start][0] >= target {
				return intervals[start]
			}
			return nil
		}

		mid := (start + end) / 2
		if intervals[mid][0] < target {
			return binSearch(target, mid+1, end)
		} else {
			return binSearch(target, start, mid)
		}
	}

	for _, interval := range intervals {
		interval2 := binSearch(interval[1], 0, len(intervals)-1)
		res[hash[ktos(interval)]] = -1
		if interval2 != nil {
			res[hash[ktos(interval)]] = hash[ktos(interval2)]
		}
	}
	return res
}

// https://leetcode.com/problems/balanced-binary-tree/editorial/
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isBalanced(root *TreeNode) bool {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	var isBalanced2 func(root *TreeNode) (bool, int)
	isBalanced2 = func(root *TreeNode) (bool, int) {
		if root == nil {
			return true, 0
		}

		lb, left := isBalanced2(root.Left)
		rb, right := isBalanced2(root.Right)
		return left-right >= -1 && left-right <= 1 && lb && rb, 1 + max(left, right)
	}
	res, _ := isBalanced2(root)
	return res
}
