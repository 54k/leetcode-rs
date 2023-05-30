package main

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/description/
func maxProfit(k int, prices []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	dp := make([][][]int, len(prices)+1)
	for i := 0; i <= len(prices); i++ {
		dp[i] = make([][]int, 2)
		for j := 0; j < 2; j++ {
			dp[i][j] = make([]int, k+1)
		}
	}

	for day := len(prices) - 1; day >= 0; day-- {
		for remain := 1; remain <= k; remain++ {
			for holding := 0; holding < 2; holding++ {
				ans := dp[day+1][holding][remain]

				if holding == 0 {
					ans = max(ans, -prices[day]+dp[day+1][1][remain])
				} else {
					ans = max(ans, prices[day]+dp[day+1][0][remain-1])
				}

				dp[day][holding][remain] = ans
			}
		}
	}

	return dp[0][0][k]
}

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/description/
func maxProfit2(prices []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	dp := make([]int, len(prices)+2)

	for i := len(prices) - 1; i >= 0; i-- {
		c1 := 0
		// case 1 buy and sell the stock
		for sell := i + 1; sell < len(prices); sell++ {
			profit := (prices[sell] - prices[i]) + dp[sell+2]
			c1 = max(c1, profit)
		}
		// case 2 do no transaction with the stock p[i]
		c2 := dp[i+1]

		dp[i] = max(c1, c2)
	}

	return dp[0]
}
