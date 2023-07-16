package main

import "fmt"

// https://leetcode.com/problems/the-number-of-good-subsets/description/
func numberOfGoodSubsets(nums []int) int {
	var PRIMES = [...]int{2, 3, 5, 7, 11, 13, 17, 19, 23, 29}
	const MOD = 1_000_000_007

	dp := make([]int, 1<<10)
	dp[0] = 1

	count := map[int]int{}
	for _, n := range nums {
		count[n]++
	}

	for a, cnt := range count {
		if a == 1 {
			continue
		}
		if a%4 == 0 || a%9 == 0 || a == 25 {
			continue
		}

		setOfPrimes := 0
		for i, p := range PRIMES {
			if a%p == 0 {
				setOfPrimes |= 1 << i
			}
		}

		for possibleSet := 0; possibleSet < 1<<10; possibleSet++ {
			if possibleSet&setOfPrimes != 0 {
				continue
			}
			resultingMask := possibleSet | setOfPrimes
			dp[resultingMask] = (dp[resultingMask] + cnt*dp[possibleSet]) % MOD
		}
	}

	sum := 0
	for _, n := range dp {
		sum += n
	}

	ones := 1
	for i := 1; i <= count[1]; i++ {
		ones *= 2
		ones %= MOD
	}

	dpSum := sum - 1

	fmt.Println(ones % MOD)
	fmt.Println(dpSum % MOD)

	return ((ones % MOD) * (dpSum % MOD)) % MOD
}
