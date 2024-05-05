package day539

import "strconv"

// https://leetcode.com/problems/restore-the-array/description/
func numberOfArrays(s string, k int) int {
	MOD := int(1e9 + 7)
	n := len(s)
	dp := make([]int, n+1)
	dp[0] = 1

	for i := 0; i < n; i++ {
		if s[i] == '0' {
			continue
		}
		for j := i; j < n; j++ {
			num, _ := strconv.Atoi(s[i : j+1])
			if num > k {
				break
			}
			dp[j+1] += dp[i]
			dp[j+1] %= MOD
		}
	}
	return dp[n]
}
