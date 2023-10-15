package day337

// https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps/description
func numWays(steps int, arrLen int) int {
	type key struct {
		pos    int
		remain int
	}
	const MOD = 1000000007
	memo := map[key]int{}
	var dp func(int, int) int
	dp = func(pos, remain int) int {
		if pos == 0 && remain == 0 {
			return 1
		}
		if pos < 0 || pos >= arrLen || remain == 0 {
			return 0
		}
		k := key{pos, remain}
		if _, ok := memo[k]; ok {
			return memo[k]
		}

		ans := 0
		ans = (ans + dp(pos+1, remain-1)) % MOD
		ans = (ans + dp(pos, remain-1)) % MOD
		ans = (ans + dp(pos-1, remain-1)) % MOD
		memo[k] = ans
		return ans
	}

	return dp(0, steps)
}
