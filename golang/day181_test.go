package main

// https://leetcode.com/problems/solving-questions-with-brainpower/description/
func max(a, b int64) int64 {
	if a >= b {
		return a
	}
	return b
}

func rec(q [][]int, i int, dp []int64) int64 {
	if i >= len(q) {
		return 0
	}
	if dp[i] == -1 {
		take := int64(q[i][0]) + rec(q, i+q[i][1]+1, dp)
		noTake := rec(q, i+1, dp)
		dp[i] = max(take, noTake)
	}
	return dp[i]
}

func mostPointsTopDown(questions [][]int) int64 {
	dp := make([]int64, len(questions)+1)
	for i := 0; i < len(dp); i++ {
		dp[i] = -1
	}
	return rec(questions, 0, dp)
}

func mostPointsBottomUp(questions [][]int) int64 {
	dp := make([]int64, len(questions)+1000000)
	for i := len(questions) - 1; i >= 0; i-- {
		dp[i] = max(int64(questions[i][0])+dp[i+1+questions[i][1]], dp[i+1])
	}
	return dp[0]
}
