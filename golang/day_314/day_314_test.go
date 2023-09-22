package day314

// https://leetcode.com/problems/median-of-two-sorted-arrays/description/
func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
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

	if len(nums1) > len(nums2) {
		return findMedianSortedArrays(nums2, nums1)
	}

	m, n := len(nums1), len(nums2)
	half := (m + n + 1) / 2

	left, right := 0, m
	for left <= right {
		partitionA := (left + right) / 2
		partitionB := half - partitionA

		maxLeftA := -(1 << 30)
		if partitionA > 0 {
			maxLeftA = nums1[partitionA-1]
		}
		minRightA := 1 << 30
		if partitionA < m {
			minRightA = nums1[partitionA]
		}

		maxLeftB := -(1 << 30)
		if partitionB > 0 {
			maxLeftB = nums2[partitionB-1]
		}
		minRightB := 1 << 30
		if partitionB < n {
			minRightB = nums2[partitionB]
		}

		if maxLeftA <= minRightB && maxLeftB <= minRightA {
			if (m+n)%2 == 0 {
				return float64(max(maxLeftA, maxLeftB)+min(minRightA, minRightB)) / 2.0
			} else {
				return float64(max(maxLeftA, maxLeftB))
			}
		} else if maxLeftA > minRightB {
			right = partitionA - 1
		} else {
			left = partitionA + 1
		}
	}

	return 0.0
}

// https://leetcode.com/problems/is-subsequence/description
func isSubsequenceDivideAndConquer(s string, t string) bool {
	leftBound, rightBound := len(s), len(t)
	var recIsSibsequeence func(int, int) bool
	recIsSibsequeence = func(leftIndex, rightIndex int) bool {
		if leftIndex == leftBound {
			return true
		}
		if rightIndex == rightBound {
			return false
		}

		if s[leftIndex] == t[rightIndex] {
			leftIndex++
		}
		rightIndex++
		return recIsSibsequeence(leftIndex, rightIndex)
	}

	return recIsSibsequeence(0, 0)
}
