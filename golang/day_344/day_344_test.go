package day344

// https://leetcode.com/problems/container-with-most-water/description/
func maxAreaBruteForce(height []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	ans := 0
	left := 0
	for left < len(height) {
		for right := left + 1; right < len(height); right++ {
			ans = max(ans, (right-left)*min(height[left], height[right]))
		}
		left++
	}
	return ans
}

func maxArea(height []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	ans := 0
	left := 0
	right := len(height) - 1

	for left < right {
		width := right - left
		h := min(height[left], height[right])
		ans = max(ans, width*h)

		if height[left] > height[right] {
			right--
		} else {
			left++
		}
	}
	return ans
}
