package day268

// https://leetcode.com/problems/search-a-2d-matrix/description/
func searchMatrix(matrix [][]int, target int) bool {
	m, n := len(matrix), len(matrix[0])
	left, right := 0, m*n-1
	for left <= right {
		mid := left + (right-left)/2
		r, c := mid/n, mid%n
		if matrix[r][c] == target {
			return true
		} else if matrix[r][c] < target {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}
	return false
}
