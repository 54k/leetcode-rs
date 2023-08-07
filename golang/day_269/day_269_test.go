package day269

// https://leetcode.com/problems/trapping-rain-water/description/
func trapBrute(height []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	ans := 0
	size := len(height)
	for i := 1; i < size-1; i++ {
		leftMax, rightMax := 0, 0
		for j := i; j >= 0; j-- {
			leftMax = max(leftMax, height[j])
		}
		for j := i; j < size; j++ {
			rightMax = max(rightMax, height[j])
		}
		ans += min(leftMax, rightMax) - height[i]
	}
	return ans
}

func trapPrefix(height []int) int {
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
	if len(height) == 0 {
		return 0
	}
	ans := 0
	size := len(height)
	leftMax := make([]int, size)
	leftMax[0] = height[0]
	rightMax := make([]int, size)
	rightMax[size-1] = height[size-1]
	for i := 1; i < size; i++ {
		leftMax[i] = max(leftMax[i-1], height[i])
	}
	for i := size - 2; i >= 0; i-- {
		rightMax[i] = max(rightMax[i+1], height[i])
	}
	for i := 1; i < size-1; i++ {
		ans += min(leftMax[i], rightMax[i]) - height[i]
	}
	return ans
}

func trap2pointers(height []int) int {
	left, right := 0, len(height)-1
	leftMax, rightMax := 0, 0
	ans := 0
	for left < right {
		if height[left] < height[right] {
			if height[left] >= leftMax {
				leftMax = height[left]
			} else {
				ans += leftMax - height[left]
			}
			left++
		} else {
			if height[right] >= rightMax {
				rightMax = height[right]
			} else {
				ans += rightMax - height[right]
			}
			right--
		}
	}
	return ans
}
