package main

// https://leetcode.com/problems/word-break/description/
func wordBreak(s string, wordDict []string) bool {
	dp := make([]bool, len(s))
	for i := 0; i < len(dp); i++ {
		for _, word := range wordDict {
			if i >= len(word)-1 && (i == len(word)-1 || dp[i-len(word)]) {
				if word == string([]byte(s)[i-len(word)+1:i+1]) {
					dp[i] = true
					break
				}
			}
		}
	}
	return dp[len(s)-1]
}
