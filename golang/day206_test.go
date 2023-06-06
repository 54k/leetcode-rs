package main

// https://leetcode.com/problems/maximize-distance-to-closest-person/description/
func maxDistToClosest(seats []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	i, j := 0, len(seats)-1
	for ; i < len(seats) && seats[i] == 0; i++ {
	}
	for ; j >= 0 && seats[j] == 0; j-- {
	}
	ans := max(i, len(seats)-j-1)
	cnt := 0
	for k := i; k <= j; k++ {
		if seats[k] == 0 {
			cnt++
		} else {
			ans = max(ans, (cnt+1)/2)
			cnt = 0
		}
	}
	return ans
}

// https://leetcode.com/problems/maximize-distance-to-closest-person/description/
func isReflected(points [][]int) bool {
	minx, maxx := 1000_000_007, -1000_000_007
	coords := make(map[int]map[int]bool)
	for _, p := range points {
		if _, ok := coords[p[0]]; !ok {
			coords[p[0]] = make(map[int]bool)
		}
		coords[p[0]][p[1]] = true
		if minx > p[0] {
			minx = p[0]
		}
		if maxx < p[0] {
			maxx = p[0]
		}
	}
	midx := minx + maxx
	for _, p := range points {
		rx := midx - p[0]
		if v, ok := coords[rx][p[1]]; !ok || !v {
			return false
		}
	}
	return true
}
