package day265

// https://leetcode.com/problems/word-break/description/
func wordBreak(s string, wordDict []string) bool {
	dp := make([]bool, len(s))
	dict := map[string]bool{}
	for _, w := range wordDict {
		dict[w] = true
	}
	for i := 0; i < len(s); i++ {
		for w, _ := range dict {
			if i >= len(w)-1 && (i == len(w)-1 || dp[i-len(w)]) {
				cur := s[i-len(w)+1 : i+1]
				if w == cur {
					dp[i] = true
					break
				}
			}
		}
	}
	return dp[len(s)-1]
}

// https://leetcode.com/problems/greatest-common-divisor-of-strings/description/
func gcdOfStrings(str1 string, str2 string) string {
	panic("todo")
}

// https://leetcode.com/problems/extra-characters-in-a-string/description/
func minExtraChar(s string, dictionary []string) int {
	panic("todo")
}
