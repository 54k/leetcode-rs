package day293

import "math"

// https://leetcode.com/problems/minimum-number-of-taps-to-open-to-water-a-garden/
func minTapsDP(n int, ranges []int) int {
	const INF = 1000_000_009
	dp := make([]int, n+1)
	for i := 1; i < len(dp); i++ {
		dp[i] = INF
	}

	for i, r := range ranges {
		left := int(math.Max(0, float64(i-r)))
		right := int(math.Min(float64(n), float64(r+i)))
		for j := left; j <= right; j++ {
			dp[right] = int(math.Min(float64(dp[right]), float64(dp[j]+1)))
		}
	}
	if dp[n] == INF {
		return -1
	}
	return dp[n]
}

func minTaps(n int, ranges []int) int {
	coverage := make([]int, n+1)
	for i, r := range ranges {
		left := int(math.Max(0, float64(i-r)))
		right := int(math.Min(float64(n), float64(r+i)))
		if coverage[left] < right {
			coverage[left] = right
		}
	}
	count, curEnd, nextEnd := 0, 0, 0
	for i := 0; i <= n; i++ {
		if i > nextEnd {
			return -1
		}
		if i > curEnd {
			count++
			curEnd = nextEnd
		}
		if nextEnd < coverage[i] {
			nextEnd = coverage[i]
		}
	}
	return count
}
