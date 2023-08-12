package day273

// https://leetcode.com/problems/amount-of-new-area-painted-each-day/description/
func amountPainted(paint [][]int) []int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	line := make([]int, 50001)
	res := make([]int, len(paint))

	for i := 0; i < len(paint); i++ {
		start_i, end_i := paint[i][0], paint[i][1]

		for start_i < end_i {
			jump := max(start_i+1, line[start_i])
			if line[start_i] == 0 {
				res[i] += 1
			}
			line[start_i] = max(line[start_i], end_i) // compression
			start_i = jump
		}
	}
	return res
}
