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

// https://leetcode.com/problems/destroy-sequential-targets/description/
func destroyTargets(nums []int, space int) int {
	m := map[int]int{}
	max := -(1 << 31)
	for _, n := range nums {
		m[n%space]++
		if max < m[n%space] {
			max = m[n%space]
		}
	}

	ans := 1 << 31
	for _, n := range nums {
		if m[n%space] == max && n < ans {
			ans = n
		}
	}

	return ans
}
