package day284

import "strconv"

// https://leetcode.com/problems/perfect-rectangle/description/
func isRectangleCover(rectangles [][]int) bool {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	key := func(x, y int) string {
		k := ""
		k += strconv.Itoa(x) + " " + strconv.Itoa(y)
		return k
	}

	x1 := 1 << 31
	y1 := 1 << 31
	x2 := -(1 << 31)
	y2 := -(1 << 31)

	area := 0

	set := map[string]bool{}

	add := func(x, y int) {
		k := key(x, y)
		if _, ok := set[k]; !ok {
			set[k] = true
			return
		}
		delete(set, k)
	}

	for _, rect := range rectangles {
		rx1, ry1, rx2, ry2 := rect[0], rect[1], rect[2], rect[3]
		area += (rx2 - rx1) * (ry2 - ry1)

		x1 = min(x1, rx1)
		y1 = min(y1, ry1)
		x2 = max(x2, rx2)
		y2 = max(y2, ry2)

		add(rx1, ry1)
		add(rx2, ry2)
		add(rx1, ry2)
		add(rx2, ry1)
	}

	if len(set) != 4 {
		return false
	}

	if _, ok := set[key(x1, y1)]; !ok {
		return false
	}
	if _, ok := set[key(x2, y2)]; !ok {
		return false
	}
	if _, ok := set[key(x1, y2)]; !ok {
		return false
	}
	if _, ok := set[key(x2, y1)]; !ok {
		return false
	}

	return area == (x2-x1)*(y2-y1)
}
