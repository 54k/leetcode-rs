package day323

// https://leetcode.com/problems/palindrome-pairs/description/
func palindromePairs(words []string) [][]int {
	type TrieNode struct {
		wordEnding                int
		next                      map[rune]*TrieNode
		palindromePrefixRemaining []int
	}

	hasPalindromeRemaining := func(s string, i int) bool {
		p1 := i
		p2 := len(s) - 1
		for p1 < p2 {
			if s[p1] != s[p2] {
				return false
			}
			p1++
			p2--
		}
		return true
	}

	rev := func(s string) string {
		b := []byte(s)
		for i := 0; i < len(b)/2; i++ {
			b[i], b[len(b)-1-i] = b[len(b)-1-i], b[i]
		}
		return string(b)
	}

	trie := &TrieNode{-1, map[rune]*TrieNode{}, []int{}}

	for wordId, word := range words {
		reversedWord := rev(word)
		currentTrieLevel := trie
		for j, ch := range reversedWord {
			if hasPalindromeRemaining(reversedWord, j) {
				currentTrieLevel.palindromePrefixRemaining = append(currentTrieLevel.palindromePrefixRemaining, wordId)
			}
			if _, ok := currentTrieLevel.next[ch]; !ok {
				currentTrieLevel.next[ch] = &TrieNode{-1, map[rune]*TrieNode{}, []int{}}
			}
			currentTrieLevel = currentTrieLevel.next[ch]
		}
		currentTrieLevel.wordEnding = wordId
	}

	pairs := [][]int{}
	for wordId, word := range words {
		currentTrieLevel := trie
		for j, ch := range word {
			// 3
			if currentTrieLevel.wordEnding != -1 && hasPalindromeRemaining(word, j) {
				pairs = append(pairs, []int{wordId, currentTrieLevel.wordEnding})
			}
			currentTrieLevel = currentTrieLevel.next[ch]
			if currentTrieLevel == nil {
				break
			}
		}

		if currentTrieLevel == nil {
			continue
		}
		// 1
		if currentTrieLevel.wordEnding != -1 && currentTrieLevel.wordEnding != wordId {
			pairs = append(pairs, []int{wordId, currentTrieLevel.wordEnding})
		}
		// 2
		for _, other := range currentTrieLevel.palindromePrefixRemaining {
			pairs = append(pairs, []int{wordId, other})
		}
	}

	return pairs
}
