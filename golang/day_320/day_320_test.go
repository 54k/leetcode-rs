package day320

// https://leetcode.com/problems/sort-array-by-parity/description
func sortArrayByParityLomuto(nums []int) []int {
	left := 0
	for right := 0; right < len(nums); right++ {
		if nums[right]&1 == 0 {
			nums[left], nums[right] = nums[right], nums[left]
			left++
		}
	}
	return nums
}

func sortArrayByParityHoare(nums []int) []int {
	left, right := 0, len(nums)-1
	for left != right {
		for left != right && nums[left]&1 == 0 {
			left++
		}
		for left != right && nums[right]&1 == 1 {
			right--
		}
		nums[left], nums[right] = nums[right], nums[left]
	}
	return nums
}
