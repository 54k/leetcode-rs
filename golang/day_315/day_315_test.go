package day315

import "sort"

// https://leetcode.com/problems/shortest-way-to-form-string/description/
func shortestWay(source string, target string) int {
	nextOccurences := make([][]int, len(source))
	for i := 0; i < len(source); i++ {
		nextOccurences[i] = make([]int, 26)
	}

	for j := 0; j < 26; j++ {
		nextOccurences[len(source)-1][j] = -1
	}
	nextOccurences[len(source)-1][source[len(source)-1]-'a'] = len(source) - 1

	for i := len(source) - 2; i >= 0; i-- {
		for j := 0; j < 26; j++ {
			nextOccurences[i][j] = nextOccurences[i+1][j]
		}
		nextOccurences[i][source[i]-'a'] = i
	}

	sourceIterator := 0
	count := 1
	for _, c := range target {
		if nextOccurences[0][c-'a'] == -1 {
			return -1
		}

		if sourceIterator == len(source) || nextOccurences[sourceIterator][c-'a'] == -1 {
			count++
			sourceIterator = 0
		}

		sourceIterator = nextOccurences[sourceIterator][c-'a'] + 1
	}

	return count
}

// https://leetcode.com/problems/longest-string-chain/description
func longestStrChain(words []string) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	isSubseq := func(s, t string) bool {
		i := 0
		for _, c := range t {
			if c == rune(s[i]) {
				i++
			}
			if i == len(s) {
				return true
			}
		}
		return false
	}

	sort.Slice(words, func(i, j int) bool {
		return len(words[i]) < len(words[j])
	})

	n := len(words)

	ans := 1
	dp := make([]int, n)
	for i := 0; i < n; i++ {
		dp[i] = max(dp[i], 1)
		for j := 0; j < i; j++ {
			if len(words[j])+1 == len(words[i]) && isSubseq(words[j], words[i]) {
				dp[i] = max(dp[i], dp[j]+1)
				ans = max(ans, dp[i])
			}
		}
	}
	return ans
}

// https://leetcode.com/problems/minimum-window-subsequence/description/
func minWindow(s1 string, s2 string) string {
	n, m := len(s1), len(s2)

	g := make([]int, m+1)
	f := make([]int, m+1)
	for i := 0; i < len(f); i++ {
		f[i] = int(10e6)
	}
	f[0] = 0
	end := 0
	length := n + 1

	for i := 1; i <= n; i++ {
		g[0] = 0
		for j := 1; j <= m; j++ {
			if s1[i-1] == s2[j-1] {
				g[j] = f[j-1] + 1
			} else {
				g[j] = f[j] + 1
			}
		}
		copy(f, g)
		if f[m] < length {
			length = f[m]
			end = i
		}
	}
	if length > n {
		return ""
	}
	return s1[end-length : end]
}

// https://leetcode.com/problems/find-root-of-n-ary-tree/description/
type Node struct {
	Val      int
	Children []*Node
}

func findRootExtraSpace(tree []*Node) *Node {
	seen := map[*Node]bool{}
	for _, n := range tree {
		for _, c := range n.Children {
			seen[c] = true
		}
	}
	for _, n := range tree {
		if _, ok := seen[n]; !ok {
			return n
		}
	}
	return nil
}

func findRootConstantSpace(tree []*Node) *Node {
	valueSum := 0
	for _, n := range tree {
		valueSum += n.Val // or use XOR
		for _, c := range n.Children {
			valueSum -= c.Val // or use XOR
		}
	}

	for _, n := range tree {
		if n.Val == valueSum {
			return n
		}
	}
	return nil
}
