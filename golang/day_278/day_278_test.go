package day278

// https://leetcode.com/problems/01-matrix/description/
func updateMatrix(mat [][]int) [][]int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	m, n := len(mat), len(mat[0])
	dp := make([][]int, len(mat))
	for i := 0; i < len(mat); i++ {
		dp[i] = make([]int, len(mat[0]))
	}

	for r := 0; r < len(mat); r++ {
		for c := 0; c < len(mat[0]); c++ {
			dp[r][c] = mat[r][c]
		}
	}

	for r := 0; r < len(mat); r++ {
		for c := 0; c < len(mat[0]); c++ {
			if dp[r][c] == 0 {
				continue
			}

			minNeighbor := m * n
			if r > 0 {
				minNeighbor = min(minNeighbor, dp[r-1][c])
			}

			if c > 0 {
				minNeighbor = min(minNeighbor, dp[r][c-1])
			}

			dp[r][c] = minNeighbor + 1
		}
	}

	for r := len(mat) - 1; r >= 0; r-- {
		for c := len(mat[0]) - 1; c >= 0; c-- {
			if dp[r][c] == 0 {
				continue
			}

			minNeighbor := m * n
			if r < len(mat)-1 {
				minNeighbor = min(minNeighbor, dp[r+1][c])
			}
			if c < len(mat[0])-1 {
				minNeighbor = min(minNeighbor, dp[r][c+1])
			}

			dp[r][c] = min(dp[r][c], minNeighbor+1)
		}
	}

	return dp
}

// https://leetcode.com/problems/longest-palindromic-substring/description/
func longestPalindrome(s string) string {
	expand := func(left int, right int) int {
		for left >= 0 && right < len(s) && s[left] == s[right] {
			left -= 1
			right += 1
		}
		return right - left - 1
	}

	ans := make([]int, 2)

	for i := 0; i < len(s); i++ {
		oddLength := expand(i, i)
		if oddLength > ans[1]-ans[0]+1 {
			dist := oddLength / 2
			ans[0] = i - dist
			ans[1] = i + dist
		}

		evenLength := expand(i, i+1)
		if evenLength > ans[1]-ans[0]+1 {
			dist := evenLength/2 - 1
			ans[0] = i - dist
			ans[1] = i + 1 + dist
		}
	}

	i := ans[0]
	j := ans[1]
	return s[i : j+1]
}
