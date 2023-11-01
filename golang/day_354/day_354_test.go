package day354

// https://leetcode.com/problems/edit-distance/description/
func minDistance(word1 string, word2 string) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	n := len(word1)
	m := len(word2)

	memo := make([][]int, n)
	for i := 0; i < n; i++ {
		memo[i] = make([]int, m)
		for j := 0; j < m; j++ {
			memo[i][j] = -1
		}
	}

	var dfs func(int, int) int
	dfs = func(w1 int, w2 int) int {
		if n == w1 || m == w2 {
			return n - w1 + m - w2
		}

		if memo[w1][w2] != -1 {
			return memo[w1][w2]
		}

		ans := 0
		if word1[w1] == word2[w2] {
			ans = dfs(w1+1, w2+1)
		} else {
			ans = min(dfs(w1+1, w2), min(dfs(w1+1, w2+1), dfs(w1, w2+1))) + 1
		}

		memo[w1][w2] = ans
		return ans
	}

	return dfs(0, 0)
}
