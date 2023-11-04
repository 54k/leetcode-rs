package day357

// https://leetcode.com/problems/min-cost-climbing-stairs/description/
func minCostClimbingStairs(cost []int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	costA, costB := cost[0], cost[1]
	for i := 2; i < len(cost); i++ {
		costC := min(costA, costB) + cost[i]
		costA, costB = costB, costC
	}
	return min(costA, costB)
}
