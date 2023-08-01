package day262

import "sort"

// https://leetcode.com/problems/maximum-beauty-of-an-array-after-applying-operation/
func maximumBeauty(nums []int, k int) int {
	max := func(a, b int) int {
		if a < b {
			return b
		}
		return a
	}
	sort.Ints(nums)
	left := 0
	ans := 0
	for right := 0; right < len(nums); right++ {
		for nums[right]-nums[left] > 2*k {
			left++
		}
		ans = max(ans, right-left+1)
	}
	return ans
}

// https://leetcode.com/problems/length-of-the-longest-valid-substring/
func longestValidSubstring(word string, forbidden []string) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	forbiddenSet := map[string]bool{}
	for _, s := range forbidden {
		forbiddenSet[s] = true
	}
	cur := ""
	ans := 0
	for _, ch := range word {
		cur += string(ch)
		m := len(cur)
		left := -1
		for i := max(0, m-10); i < m; i++ {
			tmp := cur[i:]

			if _, ok := forbiddenSet[tmp]; ok {
				left = i
			}
		}

		if left > -1 {
			cur = cur[left+1:]
		}

		ans = max(ans, len(cur))
	}
	return ans
}
