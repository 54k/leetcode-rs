package day327

// https://leetcode.com/problems/check-if-a-number-is-majority-element-in-a-sorted-array/description/
func isMajorityElement(nums []int, target int) bool {
	index, lo, hi := len(nums), 0, len(nums)-1

	for lo < hi {
		mid := (lo + hi) / 2
		if nums[mid] >= target {
			hi = mid
			index = mid
		} else {
			lo = mid + 1
		}
	}
	if index+len(nums)/2 >= len(nums) {
		return false
	}
	return nums[index+len(nums)/2] == target
}
