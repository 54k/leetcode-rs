package day333

import "sort"

// https://leetcode.com/problems/number-of-flowers-in-full-bloom/description/
func fullBloomFlowers(flowers [][]int, people []int) []int {
	upperBound := func(arr []int, target int) int {
		lo, hi := 0, len(arr)
		for lo < hi {
			mid := (lo + hi) / 2
			if arr[mid] <= target {
				lo = mid + 1
			} else {
				hi = mid
			}
		}
		return lo
	}
	startBloom := []int{}
	endBloom := []int{}

	for _, f := range flowers {
		startBloom = append(startBloom, f[0])
		endBloom = append(endBloom, f[1]+1)
	}
	sort.Ints(startBloom)
	sort.Ints(endBloom)

	ans := make([]int, len(people))
	for i, person := range people {
		s := upperBound(startBloom, person)
		e := upperBound(endBloom, person)
		ans[i] = s - e
	}
	return ans
}
