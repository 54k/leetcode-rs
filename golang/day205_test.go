package main

// https://leetcode.com/problems/check-if-it-is-a-straight-line/description/
func checkStraightLine(coordinates [][]int) bool {
	getXDiff := func(a, b []int) int {
		return a[0] - b[0]
	}
	getYDiff := func(a, b []int) int {
		return a[1] - b[1]
	}
	n := len(coordinates)
	deltaX, deltaY := getXDiff(coordinates[1], coordinates[0]), getYDiff(coordinates[1], coordinates[0])
	for i := 2; i < n; i++ {
		if deltaY*getXDiff(coordinates[i], coordinates[0]) !=
			deltaX*getYDiff(coordinates[i], coordinates[0]) {
			return false
		}
	}
	return true
}

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/description/
func maxProfitWithFee(prices []int, fee int) int {
	max := func(x, y int) int {
		if x > y {
			return x
		}
		return y
	}
	n := len(prices)
	dp := make([][]int, len(prices)+1)
	dp[0] = make([]int, 2)
	dp[0][0] = -prices[0]

	for i := 1; i < len(dp); i++ {
		dp[i] = make([]int, 2)
		for j := 0; j < 2; j++ {
			if j == 0 {
				// if hold, sell
				dp[i][1] = max(dp[i-1][1], dp[i-1][0]+prices[i-1]-fee)
			} else {
				// if no hold, buy
				dp[i][0] = max(dp[i-1][0], dp[i-1][1]-prices[i-1])
			}
		}
	}
	return max(dp[n][0], dp[n][1])
}
