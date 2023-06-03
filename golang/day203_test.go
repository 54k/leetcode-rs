package main

// https://leetcode.com/problems/unique-paths/description/
func uniquePaths(m int, n int) int {
	dp := make([]int, n)
	dp[0] = 1
	for r := 0; r < m; r++ {
		next := make([]int, n)
		for c := 0; c < n; c++ {
			next[c] += dp[c]
			if c > 0 {
				next[c] += next[c-1]
			}
		}
		dp = next
	}
	return dp[n-1]
}

// https://leetcode.com/problems/unique-paths-ii/description/
func uniquePathsWithObstacles(obstacleGrid [][]int) int {
	dp := [100]int{}
	dp[0] = 1
	for r := 0; r < len(obstacleGrid); r++ {
		next := [100]int{}
		for c := 0; c < len(obstacleGrid[0]); c++ {
			if obstacleGrid[r][c] == 0 {
				next[c] += dp[c]
				if c > 0 && obstacleGrid[r][c-1] == 0 {
					next[c] += next[c-1]
				}
			}
		}
		dp = next
	}
	return dp[len(obstacleGrid[0])-1]
}
