package day328

// https://leetcode.com/problems/integer-break/description/
func integerBreak(n int) int {
	memo := map[int]int{}

	var dp func(int) int
	dp = func(num int) int {
		if num <= 3 {
			return num
		}

		if _, ok := memo[num]; ok {
			return memo[num]
		}

		ans := num // don't split at all

		for i := 2; i < num; i++ {
			split := i * dp(num-i)
			if split > ans {
				ans = split
			}
		}
		memo[num] = ans
		return ans
	}

	if n <= 3 {
		return n - 1
	}
	return dp(n)
}
