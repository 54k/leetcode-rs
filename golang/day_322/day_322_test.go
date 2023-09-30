package day322

// https://leetcode.com/problems/132-pattern/description
func find132pattern(nums []int) bool {
	mini := 1 << 31
	for j := 0; j < len(nums)-1; j++ {
		if mini > nums[j] {
			mini = nums[j]
		}
		for k := j + 1; k < len(nums); k++ {
			if nums[k] < nums[j] && mini < nums[k] {
				return true
			}
		}
	}
	return false
}
