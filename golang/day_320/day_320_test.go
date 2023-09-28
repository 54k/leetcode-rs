package day320

// https://leetcode.com/problems/sort-array-by-parity/description
func sortArrayByParity(nums []int) []int {
	left := 0
	for right := 0; right < len(nums); right++ {
		if nums[right]&1 == 0 {
			nums[left], nums[right] = nums[right], nums[left]
			left++
		}
	}
	return nums
}
