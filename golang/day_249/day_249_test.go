package day249

import "sort"

// https://leetcode.com/problems/non-overlapping-intervals/description/
func eraseOverlapIntervals(intervals [][]int) int {
	sort.Slice(intervals, func(a, b int) bool {
		return intervals[a][1] < intervals[b][1]
	})
	k := -(1 << 30)
	ans := 0
	for i := 0; i < len(intervals); i++ {
		x := intervals[i][0]
		y := intervals[i][1]

		if x >= k {
			k = y
		} else {
			ans++
		}
	}
	return ans
}

// https://leetcode.com/problems/find-the-prefix-common-array-of-two-arrays/description/
func findThePrefixCommonArray(A []int, B []int) []int {
	popCount := func(n int) int {
		ans := 0
		for n != 0 {
			ans += n & 1
			n >>= 1
		}
		return ans
	}

	n := len(A)
	countA := 0
	countB := 0

	ans := []int{}

	for i := 0; i < n; i++ {
		countA |= 1 << A[i]
		countB |= 1 << B[i]
		ans = append(ans, popCount(countA&countB))
	}

	return ans
}
