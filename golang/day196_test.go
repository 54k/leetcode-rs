package main

// https://leetcode.com/problems/house-robber/description/
func rob(nums []int) int {
	if len(nums) == 1 {
		return nums[0]
	}
	dp := make([]int, len(nums))
	dp[0] = nums[0]
	dp[1] = nums[0]
	if dp[1] < nums[1] {
		dp[1] = nums[1]
	}
	for i := 2; i < len(nums); i++ {
		dp[i] = dp[i-1]
		if dp[i] < nums[i]+dp[i-2] {
			dp[i] = nums[i] + dp[i-2]
		}
	}
	return dp[len(nums)-1]
}

// https://leetcode.com/problems/min-cost-climbing-stairs/description/
func minCostClimbingStairs(cost []int) int {
	dp := make([]int, len(cost)+1)
	for i := len(cost) - 1; i >= 0; i-- {
		min := dp[i+1]
		if i < len(cost)-1 && min > dp[i+2] {
			min = dp[i+2]
		}
		dp[i] = cost[i] + min
	}
	if dp[0] < dp[1] {
		return dp[0]
	}
	return dp[1]
}

// https://leetcode.com/problems/n-th-tribonacci-number/description/
func tribonacci(n int) int {
	if n == 0 {
		return 0
	} else if n <= 2 {
		return 1
	}
	a, b, c := 0, 1, 1
	for i := 3; i <= n; i++ {
		d := a + b + c
		a, b, c = b, c, d
	}
	return c
}

// https://leetcode.com/problems/longest-common-subsequence/description/
func longestCommonSubsequence(text1 string, text2 string) int {
	max := func(a, b int) int {
		if a < b {
			return b
		}
		return a
	}
	dp := map[int]map[int]int{}
	for i := 0; i <= len(text1); i++ {
		dp[i] = map[int]int{}
	}
	for i, ch1 := range text1 {
		for j, ch2 := range text2 {
			i := i + 1
			j := j + 1
			if ch1 == ch2 {
				dp[i][j] = dp[i-1][j-1] + 1
			} else {
				dp[i][j] = max(dp[i-1][j], dp[i][j-1])
			}
		}
	}
	return dp[len(text1)][len(text2)]
}

// https://leetcode.com/problems/maximal-square/description/
func maximalSquare(matrix [][]byte) int {
	rows, cols := len(matrix), len(matrix[0])
	dp := make([][]int, rows+1)
	for i := 0; i <= rows; i++ {
		dp[i] = make([]int, cols+1)
	}

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

	maxLen := 0
	for i := 1; i <= rows; i++ {
		for j := 1; j <= cols; j++ {
			if matrix[i-1][j-1] == byte('1') {
				dp[i][j] = min(dp[i-1][j-1], min(dp[i-1][j], dp[i][j-1])) + 1
			}
			maxLen = max(maxLen, dp[i][j])
		}
	}
	return maxLen * maxLen
}
