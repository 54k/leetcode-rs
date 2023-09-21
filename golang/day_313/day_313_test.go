package day313

import (
	"fmt"
	"testing"
)

// https://leetcode.com/problems/median-of-two-sorted-arrays/description
func findMedianSortedArraysBinSearch(nums1 []int, nums2 []int) float64 {
	var solve func(int, int, int, int, int) int
	solve = func(k, as, ae, bs, be int) int {
		if as > ae {
			return nums2[k-as]
		} else if bs > be {
			return nums1[k-bs]
		}

		aMid, bMid := (as+ae)/2, (bs+be)/2
		aVal, bVal := nums1[aMid], nums2[bMid]
		// k > (len(a) + len(b)) / 2
		if aMid+bMid < k { // k in the second larger half
			if aVal < bVal { // aLeft <= aMid <= bMid <= bRight --> aLeft <= bRight
				return solve(k, aMid+1, ae, bs, be)
			} else { // Aleft <= aMid >= bMid <= bRight --> aLeft >= (bMid to bRight)
				return solve(k, as, ae, bMid+1, be)
			}
		}

		// k in the first smaller half
		if aVal < bVal {
			return solve(k, as, ae, bs, bMid-1)
		} else {
			return solve(k, as, aMid-1, bs, be)
		}
	}

	an, bn := len(nums1), len(nums2)
	n := an + bn

	if n&1 == 0 {
		a := solve(n/2, 0, an-1, 0, bn-1)
		b := solve(n/2-1, 0, an-1, 0, bn-1)
		return (float64(a) + float64(b)) / 2.0
	} else {
		return float64(solve(n/2, 0, an-1, 0, bn-1))
	}
}

// https://leetcode.com/problems/median-of-a-row-wise-sorted-matrix/description/
func matrixMedian1(grid [][]int) int {
	binSearch := func(col []int, target int) int {
		left, right := 0, len(col)-1
		for left < right {
			mid := (right + left) / 2

			if col[mid] >= target {
				right = mid
			} else {
				left = mid + 1
			}
		}

		if col[right] >= target {
			return right
		} else {
			return right + 1
		}
	}

	left, right := 1, int(10e6)
	m, n := len(grid), len(grid[0])
	target := m*n/2 + 1

	for left < right {
		mid := (right + left + 1) / 2
		sum := 0
		for _, col := range grid {
			sum += binSearch(col, mid)
		}

		if sum < target {
			right = mid - 1
		} else {
			left = mid
		}
	}

	return right
}

func matrixMedian2(grid [][]int) int {
	binSearch := func(col []int, target int) int {
		left, right := 0, len(col)-1
		for left <= right {
			mid := (right + left) / 2

			if col[mid] >= target {
				right = mid - 1
			} else {
				left = mid + 1
			}
		}
		return left
	}

	left, right := 1, int(10e6)
	m, n := len(grid), len(grid[0])
	target := m * n / 2

	for left+1 < right {
		mid := (right + left) / 2
		sum := 0
		for _, col := range grid {
			sum += n - binSearch(col, mid)
		}

		if sum > target {
			left = mid
		} else {
			right = mid
		}
	}

	return left
}

func matrixMedian3(grid [][]int) int {
	binSearch := func(col []int, target int) int {
		left, right := 0, len(col)-1
		for left <= right {
			mid := (right + left) / 2

			if col[mid] >= target {
				right = mid - 1
			} else {
				left = mid + 1
			}
		}
		return left
	}

	left, right := 1, int(10e6)
	m, n := len(grid), len(grid[0])
	target := m * n / 2

	for left+1 < right {
		mid := (right + left) / 2
		sum := 0
		for _, col := range grid {
			sum += binSearch(col, mid)
		}

		if sum <= target {
			left = mid
		} else {
			right = mid
		}
	}

	return left
}

func TestMatrixMedian(t *testing.T) {
	res := matrixMedian3([][]int{{1, 1, 2}, {2, 3, 3}, {1, 3, 4}})
	fmt.Println(res) // 2
}
