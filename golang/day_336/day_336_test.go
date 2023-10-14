package day336

// https://leetcode.com/problems/painting-the-walls/description
func paintWalls(cost []int, time []int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	n := len(cost)
	memo := make([][]int, n)
	for i, _ := range memo {
		memo[i] = make([]int, n+1)
	}

	var dp func(int, int) int
	dp = func(i, remain int) int {
		if remain <= 0 {
			return 0
		}

		if i == n {
			return 1 << 30
		}

		if memo[i][remain] != 0 {
			return memo[i][remain]
		}

		paint := cost[i] + dp(i+1, remain-1-time[i])
		dontPaint := dp(i+1, remain)
		memo[i][remain] = min(paint, dontPaint)
		return memo[i][remain]
	}
	return dp(0, n)
}
