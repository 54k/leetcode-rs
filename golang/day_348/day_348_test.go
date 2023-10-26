package day348

// https://leetcode.com/problems/palindrome-partitioning/description/
func partition(s string) [][]string {
	n := len(s)
	dp := make([][]bool, n)
	for i := 0; i < n; i++ {
		dp[i] = make([]bool, n)
	}

	result := [][]string{}
	curr := []string{}

	var dfs func(int)
	dfs = func(start int) {
		if start >= len(s) {
			r := make([]string, len(curr))
			copy(r, curr)
			result = append(result, r)
		}

		for end := start; end < len(s); end++ {
			if s[start] == s[end] && (end-start <= 2 || dp[start+1][end-1]) {
				dp[start][end] = true
				curr = append(curr, s[start:end+1])
				dfs(end + 1)
				curr = curr[:len(curr)-1]
			}
		}
	}
	dfs(0)
	return result
}
