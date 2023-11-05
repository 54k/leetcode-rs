package day358

// https://leetcode.com/problems/climbing-stairs/description/
func climbStairsBottomUp(n int) int {
	a, b := 0, 1
	for i := 2; i <= n+1; i++ {
		c := a + b
		a, b = b, c
	}
	return b
}

func climbStairsBinets(n int) int {
	q := [][]int{{1, 1}, {1, 0}}

	multiply := func(a, b [][]int) [][]int {
		c := [][]int{{0, 0}, {0, 0}}
		for i := 0; i < 2; i++ {
			for j := 0; j < 2; j++ {
				c[i][j] = a[i][0]*b[0][j] + a[i][1]*b[1][j]
			}
		}
		return c
	}

	pow := func() [][]int {
		ret := [][]int{{1, 0}, {0, 1}}
		for n > 0 {
			if n&1 == 1 {
				ret = multiply(ret, q)
			}
			n >>= 1
			q = multiply(q, q)
		}
		return ret
	}

	res := pow()
	return res[0][0]
}

// https://leetcode.com/problems/fibonacci-number/description
func fib(n int) int {
	if n <= 1 {
		return n
	}
	a, b := 0, 1
	for i := 2; i <= n; i++ {
		c := a + b
		a, b = b, c
	}
	return b
}

// https://leetcode.com/problems/n-th-tribonacci-number/description
func tribonacci(n int) int {
	if n <= 1 {
		return n
	}
	if n == 2 {
		return 1
	}
	a, b, c := 0, 1, 1
	for i := 3; i <= n; i++ {
		d := a + b + c
		a, b, c = b, c, d
	}
	return c
}

// https://leetcode.com/problems/min-cost-climbing-stairs/description
func minCostClimbingStairs(cost []int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	a, b := cost[0], cost[1]
	for i := 2; i < len(cost); i++ {
		c := min(a, b) + cost[i]
		a, b = b, c
	}
	return min(a, b)
}

// https://leetcode.com/problems/house-robber/description
func rob(nums []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	if len(nums) == 1 {
		return nums[0]
	}
	a, b := nums[0], max(nums[0], nums[1])
	for i := 2; i < len(nums); i++ {
		c := max(b, a+nums[i])
		a, b = b, c
	}
	return max(a, b)
}

// https://leetcode.com/problems/delete-and-earn/description
func deleteAndEarn(nums []int) int {
	max := func(a, b int) int {
		if a < b {
			return b
		}
		return a
	}

	points := map[int]int{}
	memo := map[int]int{}
	maxNum := 0

	for _, num := range nums {
		points[num] += num
		maxNum = max(maxNum, num)
	}

	var dp func(int) int
	dp = func(curr int) int {
		if curr == 0 {
			return 0
		}

		if curr == 1 {
			return points[curr]
		}

		if v, ok := memo[curr]; ok {
			return v
		}

		gain := points[curr]
		memo[curr] = max(dp(curr-1), dp(curr-2)+gain)
		return memo[curr]
	}

	return dp(maxNum)
}
