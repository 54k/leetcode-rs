package main

// https://leetcode.com/problems/longest-arithmetic-subsequence-of-given-difference/description/
func longestSubsequence(arr []int, difference int) int {
	dp := map[int]int{}
	answer := 1

	for _, a := range arr {
		beforeA := dp[a-difference]
		dp[a] = beforeA + 1
		if dp[a] > answer {
			answer = dp[a]
		}
	}

	return answer
}
