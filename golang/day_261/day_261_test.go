package day261

// https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/description/
func minimumDeleteSum(s1 string, s2 string) int {
	m, n := len(s1), len(s2)
	dp := make([][]int, m+1)
	for i := 0; i <= m; i++ {
		dp[i] = make([]int, n+1)
	}
	for i := 1; i <= m; i++ {
		dp[i][0] = dp[i-1][0] + int(s1[i-1])
	}
	for j := 1; j <= n; j++ {
		dp[0][j] = dp[0][j-1] + int(s2[j-1])
	}

	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			if s1[i-1] == s2[j-1] {
				dp[i][j] = dp[i-1][j-1]
				continue
			}

			delOne := int(s1[i-1]) + dp[i-1][j]
			delTwo := int(s2[j-1]) + dp[i][j-1]

			dp[i][j] = delOne
			if dp[i][j] > delTwo {
				dp[i][j] = delTwo
			}
		}
	}

	return dp[m][n]
}
