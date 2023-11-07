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
