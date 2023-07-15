package main

import (
	"sort"
)

// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii/description/
func maxValueTopDown(events [][]int, k int) int {
	sort.Slice(events[:], func(i, j int) bool {
		return events[i][0] < events[j][0]
	})
	memo := make([][]int, len(events)+1)
	for i := 0; i <= len(events); i++ {
		memo[i] = make([]int, k+1)
		for j := 0; j <= k; j++ {
			memo[i][j] = -1
		}
	}

	bisect := func(target int) int {
		left, right := 0, len(events)
		for left < right {
			mid := (left + right) / 2
			if events[mid][0] <= target {
				left = mid + 1
			} else {
				right = mid
			}
		}
		return left
	}

	var dfs func(int, int) int
	dfs = func(cur, count int) int {
		if cur == len(events) || count == 0 {
			return 0
		}

		if memo[cur][count] != -1 {
			return memo[cur][count]
		}

		idx := bisect(events[cur][1])

		memo[cur][count] = events[cur][2] + dfs(idx, count-1)
		noTake := dfs(cur+1, count)

		if noTake > memo[cur][count] {
			memo[cur][count] = noTake
		}

		return memo[cur][count]
	}

	return dfs(0, k)
}

func maxValueBottomUp(events [][]int, k int) int {
	sort.Slice(events[:], func(i, j int) bool {
		return events[i][0] < events[j][0]
	})

	dp := make([][]int, len(events)+1)
	for i := 0; i <= len(events); i++ {
		dp[i] = make([]int, k+1)
	}

	bisect := func(target int) int {
		left, right := 0, len(events)
		for left < right {
			mid := (left + right) / 2
			if events[mid][0] <= target {
				left = mid + 1
			} else {
				right = mid
			}
		}
		return left
	}

	for cur := len(events) - 1; cur >= 0; cur-- {
		idx := bisect(events[cur][1])
		for count := 1; count <= k; count++ {
			dp[cur][count] = dp[cur+1][count]
			take := dp[idx][count-1] + events[cur][2]
			if take > dp[cur][count] {
				dp[cur][count] = take
			}
		}
	}

	return dp[0][k]
}
