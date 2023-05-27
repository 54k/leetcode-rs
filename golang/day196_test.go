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
