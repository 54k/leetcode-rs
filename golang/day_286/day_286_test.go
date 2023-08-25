package day286

import "sort"

// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/description/
func smallerNumbersThanCurrent(nums []int) []int {
	nums2 := make([]int, len(nums))
	copy(nums2, nums)
	sort.Ints(nums2)
	lastIdx := map[int]int{}

	for i := len(nums2) - 1; i >= 0; i-- {
		lastIdx[nums2[i]] = i
	}

	ans := make([]int, len(nums))

	for i := len(nums) - 1; i >= 0; i-- {
		ans[i] = lastIdx[nums[i]]
	}

	return ans
}
