package day370

import (
	"math"
)

// todo not work
// https://leetcode.com/problems/continuous-subarrays/
func continuousSubarrays(nums []int) int64 {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	n := len(nums)
	lg := int(math.Log(float64(n)) + 1)

	stMin := make([][]int, n)
	for i := 0; i < n; i++ {
		stMin[i] = make([]int, lg)
		stMin[i][0] = nums[i]
	}

	for j := 1; j < lg; j++ {
		for i := 0; i+(1<<(j-1)) < n; i++ {
			stMin[i][j] = min(stMin[i][j-1], stMin[i+(1<<(j-1))][j-1])
		}
	}

	stMax := make([][]int, n)
	for i := 0; i < n; i++ {
		stMax[i] = make([]int, lg)
		stMax[i][0] = nums[i]
	}

	for j := 1; j < lg; j++ {
		for i := 0; i+(1<<(j-1)) < n; i++ {
			stMax[i][j] = max(stMax[i][j-1], stMax[i+(1<<(j-1))][j-1])
		}
	}

	ans := int64(0)
	l := 0
	for r := 0; r < n; r++ {
		j := int(math.Log(float64(r - l + 1)))

		for max(stMax[l][j], stMax[r-(1<<j)+1][j])-min(stMin[l][j], stMin[r-(1<<j)+1][j]) > 2 {
			l++
		}

		ans += int64(r - l + 1)
	}
	return ans

}
