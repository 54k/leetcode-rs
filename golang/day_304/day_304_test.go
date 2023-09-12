package day304

import (
	"fmt"
	"sort"
	"testing"
)

// https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique/
func minDeletionsSet(s string) int {
	freq := map[rune]int{}
	for _, ch := range s {
		freq[ch]++
	}
	seen := map[int]bool{}
	minimumDels := 0
	for _, ch := range "abcdefghijklmnopqrstuvwxyz" {
		for freq[ch] > 0 && seen[freq[ch]] {
			freq[ch]--
			minimumDels++
		}
		seen[freq[ch]] = true
	}
	return minimumDels
}

func minDeletionsSort(s string) int {
	freq := make([]int, 26)
	for _, ch := range s {
		freq[ch-'a']++
	}

	sort.Ints(freq)
	minDeletions := 0
	maxFreq := len(s)

	for i := len(freq) - 1; i >= 0; i-- {
		if maxFreq < freq[i] {
			minDeletions += freq[i] - maxFreq
			freq[i] = maxFreq
		}
		maxFreq = 0
		if freq[i]-1 > 0 {
			maxFreq = freq[i] - 1
		}
	}

	return minDeletions
}

func TestMinDeletions(t *testing.T) {
	res := minDeletionsSort("aab")
	fmt.Println(res)
}
