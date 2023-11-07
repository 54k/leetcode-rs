package day360

import "sort"

// https://leetcode.com/problems/eliminate-maximum-number-of-monsters/description
func eliminateMaximum(dist []int, speed []int) int {
	at := make([]float64, len(dist))
	for i, d := range dist {
		at[i] = float64(d) / float64(speed[i])
	}

	sort.Slice(at, func(a, b int) bool {
		return at[a] < at[b]
	})

	ans := 0.0
	for _, time := range at {
		if time <= ans {
			return int(ans)
		}
		ans += 1.0
	}
	return int(ans)
}

// https://leetcode.com/problems/longest-palindromic-substring/description/
func longestPalindromeDP1(s string) string {
	palindromeDp := make([][]bool, len(s))
	for i := 0; i < len(s); i++ {
		palindromeDp[i] = make([]bool, len(s))
	}

	start, end := 0, 0

	for i := len(s) - 1; i >= 0; i-- {
		for j := i + 1; j < len(s); j++ {
			palindromeDp[i][j] = s[i] == s[j] && (j-i <= 2 || palindromeDp[i+1][j-1])
			if palindromeDp[i][j] && j-i > end-start {
				start = i
				end = j
			}
		}
	}

	return s[start : end+1]
}

func longestPalindromeDP2(s string) string {
	palindromeDp := make([][]bool, len(s))
	for i := 0; i < len(s); i++ {
		palindromeDp[i] = make([]bool, len(s))
	}

	start, end := 0, 0

	for j := 0; j < len(s); j++ {
		for i := 0; i <= j; i++ {
			palindromeDp[i][j] = s[i] == s[j] && (j-i <= 2 || palindromeDp[i+1][j-1])
			if palindromeDp[i][j] && j-i > end-start {
				start = i
				end = j
			}
		}
	}

	return s[start : end+1]
}

func longestPalindromeDP3(s string) string {
	palindromeDp := make([][]bool, len(s))
	for i := 0; i < len(s); i++ {
		palindromeDp[i] = make([]bool, len(s))
	}

	start, end := 0, 0

	for i := 0; i < len(s); i++ {
		palindromeDp[i][i] = true
	}

	for i := 0; i < len(s)-1; i++ {
		if s[i] == s[i+1] {
			palindromeDp[i][i+1] = true
			start = i
			end = i + 1
		}
	}

	for diff := 2; diff < len(s); diff++ {
		for i := 0; i < len(s)-diff; i++ {
			j := i + diff
			palindromeDp[i][j] = s[i] == s[j] && palindromeDp[i+1][j-1]
			if palindromeDp[i][j] && end-start < j-i {
				start = i
				end = j
			}
		}
	}

	return s[start : end+1]
}

func longestPalindromeExpandArountCenter(s string) string {
	start, end := 0, 0
	for i := 0; i < len(s); i++ {
		for j := 0; j <= 1; j++ {
			left := i
			right := i + j

			for 0 <= left && len(s) > right && s[left] == s[right] {
				left--
				right++
			}
			left++
			right--

			if end-start < right-left {
				start = left
				end = right
			}
		}
	}
	return s[start : end+1]
}

// https://leetcode.com/problems/palindromic-substrings/description/
func countSubstringsDP1(s string) int {
	dp := make([][]bool, len(s))
	for i := 0; i < len(s); i++ {
		dp[i] = make([]bool, len(s))
	}

	ans := 0
	for i := len(s) - 1; i >= 0; i-- {
		for j := i; j < len(s); j++ {
			if s[i] == s[j] && (j-i <= 2 || dp[i+1][j-1]) {
				dp[i][j] = true
				ans++
			}
		}
	}
	return ans
}

func countSubstringsDP2(s string) int {
	dp := make([][]bool, len(s))
	for i := 0; i < len(s); i++ {
		dp[i] = make([]bool, len(s))
	}

	ans := 0
	for end := 0; end < len(s); end++ {
		for start := 0; start <= end; start++ {
			if s[start] == s[end] && (end-start <= 2 || dp[start+1][end-1]) {
				dp[start][end] = true
				ans++
			}
		}
	}
	return ans
}

func countSubstringsDP3(s string) int {
	dp := make([][]bool, len(s))
	for i := 0; i < len(s); i++ {
		dp[i] = make([]bool, len(s))
	}

	ans := 0
	for i := 0; i < len(s); i++ {
		dp[i][i] = true
		ans++
	}

	for i := 0; i < len(s)-1; i++ {
		if s[i] == s[i+1] {
			dp[i][i+1] = true
			ans++
		}
	}

	for diff := 2; diff < len(s); diff++ {
		for left := 0; left < len(dp)-diff; left++ {
			right := left + diff

			if s[left] == s[right] && dp[left+1][right-1] {
				dp[left][right] = true
				ans++
			}
		}
	}

	return ans
}

func countSubstringsDP4(s string) int {
	dp := make([][]bool, len(s))
	for i := 0; i < len(s); i++ {
		dp[i] = make([]bool, len(s))
	}

	ans := 0

	for i := 0; i < len(s); i++ {
		dp[i][i] = true
		ans++
	}

	for i := 0; i < len(s)-1; i++ {
		if s[i] == s[i+1] {
			dp[i][i+1] = true
			ans++
		}
	}

	for diff := 3; diff <= len(s); diff++ {
		for i := 0; i <= len(s)-diff; i++ {
			j := i + diff - 1
			if s[i] == s[j] && dp[i+1][j-1] {
				dp[i][j] = true
				ans++
			}
		}
	}

	return ans
}
