package day331

// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/description
func searchRange(nums []int, target int) []int {
	if len(nums) == 0 {
		return []int{-1, -1}
	}
	first := func() int {
		lo, hi := 0, len(nums)-1
		for lo < hi {
			mid := (lo + hi) / 2
			if nums[mid] < target {
				lo = mid + 1
			} else {
				hi = mid
			}
		}
		if nums[lo] != target {
			return -1
		}
		return lo
	}
	second := func() int {
		lo, hi := 0, len(nums)-1
		for lo+1 < hi {
			mid := (lo + hi) / 2
			if nums[mid] > target {
				hi = mid
			} else {
				lo = mid
			}
		}
		if nums[hi] == target {
			return hi
		}
		if nums[lo] == target {
			return lo
		}
		return -1
	}
	return []int{first(), second()}
}
