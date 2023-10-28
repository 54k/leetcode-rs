package day350

// https://leetcode.com/problems/count-vowels-permutation/description
func countVowelPermutationBottomUp(n int) int {
	const MOD = 1_000_000_007

	aVowelPermutationCount := make([]int, n)
	eVowelPermutationCount := make([]int, n)
	iVowelPermutationCount := make([]int, n)
	oVowelPermutationCount := make([]int, n)
	uVowelPermutationCount := make([]int, n)

	aVowelPermutationCount[0] = 1
	eVowelPermutationCount[0] = 1
	iVowelPermutationCount[0] = 1
	oVowelPermutationCount[0] = 1
	uVowelPermutationCount[0] = 1

	for i := 1; i < n; i++ {
		aVowelPermutationCount[i] = (eVowelPermutationCount[i-1] + iVowelPermutationCount[i-1] + uVowelPermutationCount[i-1]) % MOD
		eVowelPermutationCount[i] = (aVowelPermutationCount[i-1] + iVowelPermutationCount[i-1]) % MOD
		iVowelPermutationCount[i] = (eVowelPermutationCount[i-1] + oVowelPermutationCount[i-1]) % MOD
		oVowelPermutationCount[i] = iVowelPermutationCount[i-1]
		uVowelPermutationCount[i] = (iVowelPermutationCount[i-1] + oVowelPermutationCount[i-1]) % MOD
	}

	result := 0
	result = (aVowelPermutationCount[n-1] + eVowelPermutationCount[n-1] + iVowelPermutationCount[n-1] + oVowelPermutationCount[n-1] + uVowelPermutationCount[n-1]) % MOD
	return result
}

func countVowelPermutationBottomUpOptimized(n int) int {
	const MOD = 1_000_000_007

	aVowelPermutationCount := 1
	eVowelPermutationCount := 1
	iVowelPermutationCount := 1
	oVowelPermutationCount := 1
	uVowelPermutationCount := 1

	for i := 1; i < n; i++ {
		aVowelPermutationCountNew := (eVowelPermutationCount + iVowelPermutationCount + uVowelPermutationCount) % MOD
		eVowelPermutationCountNew := (aVowelPermutationCount + iVowelPermutationCount) % MOD
		iVowelPermutationCountNew := (eVowelPermutationCount + oVowelPermutationCount) % MOD
		oVowelPermutationCountNew := iVowelPermutationCount
		uVowelPermutationCountNew := (iVowelPermutationCount + oVowelPermutationCount) % MOD

		aVowelPermutationCount = aVowelPermutationCountNew
		eVowelPermutationCount = eVowelPermutationCountNew
		iVowelPermutationCount = iVowelPermutationCountNew
		oVowelPermutationCount = oVowelPermutationCountNew
		uVowelPermutationCount = uVowelPermutationCountNew

	}

	result := 0
	result = (aVowelPermutationCount + eVowelPermutationCount + iVowelPermutationCount + oVowelPermutationCount + uVowelPermutationCount) % MOD
	return result
}

func countVowelPermutationTopDown(n int) int {
	const MOD = 1_000_000_007

	memo := make([][]int, n)
	for i := 0; i < n; i++ {
		memo[i] = make([]int, 5)
	}

	var dp func(int, int) int
	dp = func(i int, vowel int) int {
		if memo[i][vowel] != 0 {
			return memo[i][vowel]
		}
		if i == 0 {
			memo[i][vowel] = 1
		} else {
			if vowel == 0 {
				memo[i][vowel] = (dp(i-1, 1) + dp(i-1, 2) + dp(i-1, 4)) % MOD
			} else if vowel == 1 {
				memo[i][vowel] = (dp(i-1, 0) + dp(i-1, 2)) % MOD
			} else if vowel == 2 {
				memo[i][vowel] = (dp(i-1, 1) + dp(i-1, 3)) % MOD
			} else if vowel == 3 {
				memo[i][vowel] = dp(i-1, 2) % MOD
			} else if vowel == 4 {
				memo[i][vowel] = (dp(i-1, 2) + dp(i-1, 3)) % MOD
			}
		}
		return memo[i][vowel]
	}

	result := 0
	for i := 0; i < 5; i++ {
		result = (result + dp(n-1, i)) % MOD
	}
	return result
}
