package day294

// https://leetcode.com/problems/extra-characters-in-a-string/
type TrieNode struct {
	children map[byte]*TrieNode
	isWord   bool
}

func newTrie() *TrieNode {
	return &TrieNode{map[byte]*TrieNode{}, false}
}

func buildTrie(dictionary []string) *TrieNode {
	root := newTrie()
	for _, word := range dictionary {
		node := root
		for _, ch := range word {
			ch := byte(ch)
			if _, ok := node.children[ch]; !ok {
				node.children[ch] = newTrie()
			}
			node = node.children[ch]
		}
		node.isWord = true
	}
	return root
}

func minExtraChar(s string, dictionary []string) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	memo := make([]int, len(s)+1)
	for i := 0; i < len(memo); i++ {
		memo[i] = -1
	}

	root := buildTrie(dictionary)

	var dp func(int) int
	dp = func(start int) int {
		if start == len(s) {
			return 0
		}
		if memo[start] != -1 {
			return memo[start]
		}

		node := root

		ans := dp(start+1) + 1
		for end := start; end < len(s); end++ {
			ch := s[end]
			if _, ok := node.children[ch]; !ok {
				break
			}
			node = node.children[ch]
			if node.isWord {
				ans = min(ans, dp(end+1))
			}
		}
		memo[start] = ans
		return ans
	}

	return dp(0)
}

// https://leetcode.com/problems/word-break/description/
type TrieNode2 struct {
	isWord   bool
	children map[rune]*TrieNode2
}

func newTrieNode2() *TrieNode2 {
	return &TrieNode2{false, map[rune]*TrieNode2{}}
}

func buildTrie2(wordDict []string) *TrieNode2 {
	root := newTrieNode2()
	for _, w := range wordDict {
		node := root
		for _, ch := range w {
			if _, ok := node.children[ch]; !ok {
				node.children[ch] = newTrieNode2()
			}
			node = node.children[ch]
		}
		node.isWord = true
	}
	return root
}

func wordBreak(s string, wordDict []string) bool {
	root := buildTrie2(wordDict)
	dp := make([]bool, len(s))

	for i := 0; i < len(s); i++ {
		if i == 0 || dp[i-1] {
			curr := root
			for j := i; j < len(s); j++ {
				ch := rune(s[j])
				if _, ok := curr.children[ch]; !ok {
					break
				}

				curr = curr.children[ch]
				if curr.isWord {
					dp[j] = true
				}
			}
		}
	}

	return dp[len(s)-1]
}
