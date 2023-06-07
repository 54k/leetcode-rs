package main

// https://leetcode.com/problems/maximum-length-of-repeated-subarray/description/
func findLength(nums1 []int, nums2 []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	dp := make([][]int, len(nums1)+1)
	for i := 0; i <= len(nums1); i++ {
		dp[i] = make([]int, len(nums2)+1)
	}
	ans := 0
	for i := len(nums1) - 1; i >= 0; i-- {
		for j := len(nums2) - 1; j >= 0; j-- {
			if nums1[i] == nums2[j] {
				dp[i][j] = dp[i+1][j+1] + 1
			}
			ans = max(ans, dp[i][j])
		}
	}
	return ans
}
