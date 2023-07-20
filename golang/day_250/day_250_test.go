package day_250

// https://leetcode.com/problems/asteroid-collision/description/
func asteroidCollision(asteroids []int) []int {
	abs := func(i int) int {
		if i < 0 {
			return -i
		}
		return i
	}
	stack := []int{}
label:
	for _, a := range asteroids {
		for len(stack) > 0 && stack[len(stack)-1] > 0 && a < 0 {
			absA, absB := abs(a), abs(stack[len(stack)-1])
			if absA < absB {
				continue label
			}
			stack = stack[:len(stack)-1]
			if absA == absB {
				continue label
			}
		}
		stack = append(stack, a)
	}
	return stack
}
