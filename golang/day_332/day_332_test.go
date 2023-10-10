package day332

import "sort"

// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-continuous/description/
func minOperations(nums []int) int {
	n := len(nums)
	uniq := map[int]bool{}
	for _, num := range nums {
		uniq[num] = true
	}
	newNum := []int{}
	for num, _ := range uniq {
		newNum = append(newNum, num)
	}

	sort.Ints(newNum)
	ans := 1 << 30
	j := 0
	for i := 0; i < len(newNum); i++ {
		for j < len(newNum) && newNum[j] < newNum[i]+n {
			j++
		}
		count := j - i
		if ans > n-count {
			ans = n - count
		}
	}
	return ans
}
