package day318

import "strings"

// https://leetcode.com/problems/remove-duplicate-letters/description
func removeDuplicateLetters(s string) string {
	cnt := make([]int, 26)
	for _, ch := range s {
		cnt[ch-'a']++
	}
	pos := 0
	for i, ch := range s {
		if ch < rune(s[pos]) {
			pos = i
		}
		cnt[rune(s[i])-'a']--
		if cnt[rune(s[i])-'a'] == 0 {
			break
		}
	}
	if len(s) == 0 {
		return ""
	}
	return string(s[pos]) + removeDuplicateLetters(strings.ReplaceAll(s[pos+1:], string(s[pos]), ""))
}
