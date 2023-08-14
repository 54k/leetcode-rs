package day275

import random "math/rand"

// https://leetcode.com/problems/kth-largest-element-in-an-array/description/
func findKthLargest(nums []int, k int) int {
	var quickselect func([]int, int) int
	quickselect = func(nums []int, k int) int {
		pivotIdx := random.Int() % len(nums)
		pivot := nums[pivotIdx]

		left := []int{}
		mid := []int{}
		right := []int{}

		for i := 0; i < len(nums); i++ {
			if nums[i] < pivot {
				right = append(right, nums[i])
			} else if nums[i] == pivot {
				mid = append(mid, nums[i])
			} else {
				left = append(left, nums[i])
			}
		}

		if len(left) >= k {
			return quickselect(left, k)
		} else if len(left)+len(mid) < k {
			return quickselect(right, k-len(left)-len(mid))
		}
		return pivot
	}
	return quickselect(nums, k)
}
