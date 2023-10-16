package day338

// https://leetcode.com/problems/pascals-triangle/description/
func generate(numRows int) [][]int {
	triangle := [][]int{}
	triangle = append(triangle, []int{1})
	for row := 1; row < numRows; row++ {
		prev := triangle[len(triangle)-1]
		next := []int{1}
		for i := 1; i < row; i++ {
			next = append(next, prev[i-1]+prev[i])
		}
		next = append(next, 1)
		triangle = append(triangle, next)
	}
	return triangle
}
