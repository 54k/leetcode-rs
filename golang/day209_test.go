package main

// https://leetcode.com/problems/domino-and-tromino-tiling/description/
const MOD = 1000000007

var SQMAT = [][]int{
	{1, 1, 2},
	{1, 0, 0},
	{0, 1, 1},
}

func mul(m1, m2 [][]int) [][]int {
	n := len(m1)
	ans := make([][]int, n)
	for i := 0; i < n; i++ {
		ans[i] = make([]int, n)
		for j := 0; j < n; j++ {
			sum := 0
			for k := 0; k < n; k++ {
				sum = (sum + m1[i][k]*m2[k][j]) % MOD
			}
			ans[i][j] = sum
		}
	}
	return ans
}

func expo(n int) int {
	cur := SQMAT
	for i := 1; i < n; i++ {
		cur = mul(cur, SQMAT)
	}
	return (cur[0][0]*2 + cur[0][1] + cur[0][2]) % MOD
}

func expo2(n int) [][]int {
	cur := SQMAT
	if n == 1 {
		cur = SQMAT
	} else if n%2 == 1 {
		cur = mul(expo2(n-1), SQMAT)
	} else {
		cur = mul(expo2(n/2), expo2(n/2))
	}
	return cur
}

func numTilings(n int) int {
	if n <= 2 {
		return n
	}
	ans := expo2(n - 2)[0]
	return (ans[0]*2 + ans[1] + ans[2]) % MOD
}
