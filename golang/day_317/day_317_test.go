package day317

import "strings"

// https://leetcode.com/problems/find-the-difference/description
func findTheDifference(s string, t string) byte {
	tMap := map[rune]int{}
	for _, ch := range t {
		tMap[ch]++
	}
	for _, ch := range s {
		tMap[ch]--
		if tMap[ch] == 0 {
			delete(tMap, ch)
		}
	}
	for k, _ := range tMap {
		return byte(k)
	}
	panic("oops")
}

// https://leetcode.com/problems/replace-words/description/
func replaceWords(dictionary []string, sentence string) string {
	type TrieNode struct {
		children map[rune]*TrieNode
		word     string
	}

	new := func() *TrieNode {
		return &TrieNode{
			children: make(map[rune]*TrieNode, 26),
		}
	}

	trie := new()
	for _, root := range dictionary {
		cur := trie
		for _, ch := range root {
			if cur.children[ch-'a'] == nil {
				cur.children[ch-'a'] = new()
			}
			cur = cur.children[ch-'a']
		}
		cur.word = root
	}

	ans := ""
	for _, w := range strings.Split(sentence, " ") {
		if len(ans) > 0 {
			ans += " "
		}

		cur := trie
		for _, ch := range w {
			if cur.children[ch-'a'] == nil || cur.word != "" {
				break
			}
			cur = cur.children[ch-'a']
		}
		if cur.word != "" {
			ans += cur.word
		} else {
			ans += w
		}
	}

	return ans
}
