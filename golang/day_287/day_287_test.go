package day287

import "sort"

func findLongestChain(pairs [][]int) int {
	sort.Slice(pairs, func(a, b int) bool {
		return pairs[a][1] < pairs[b][1]
	})

	current := 0
	last := -(1 << 31)
	for _, pair := range pairs {
		if pair[0] > last {
			last = pair[1]
			current++
		}
	}
	return current
}
