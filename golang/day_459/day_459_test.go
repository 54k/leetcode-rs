package day459

import "math"

// https://leetcode.com/problems/cherry-pickup-ii/description/
func cherryPickup(grid [][]int) int {
	m := len(grid)
	n := len(grid[0])
	dp := make([][][]int, m)
	for i := 0; i < m; i++ {
		dp[i] = make([][]int, n)
		for j := 0; j < n; j++ {
			dp[i][j] = make([]int, n)
		}
	}

	for r := m - 1; r >= 0; r-- {
		for i := 0; i < n; i++ {
			for j := 0; j < n; j++ {
				ans := 0
				ans += grid[r][i]
				if i != j {
					ans += grid[r][j]
				}

				if r < m-1 {
					max := 0
					for newCol1 := i - 1; newCol1 <= i+1; newCol1++ {
						for newCol2 := j - 1; newCol2 <= j+1; newCol2++ {
							if 0 <= newCol1 && newCol1 < n && 0 <= newCol2 && newCol2 < n {
								max = int(math.Max(float64(max), float64(dp[r+1][newCol1][newCol2])))
							}
						}
					}
					ans += max
				}

				dp[r][i][j] = ans
			}
		}
	}

	return dp[0][0][n-1]
}
