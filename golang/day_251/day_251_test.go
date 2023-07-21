package day251

// https://leetcode.com/problems/number-of-longest-increasing-subsequence/description/
func findNumberOfLIS(nums []int) int {
	length := make([]int, len(nums))
	count := make([]int, len(nums))

	for i := 0; i < len(nums); i++ {
		length[i] = 1
		count[i] = 1
	}

	for i := 0; i < len(nums); i++ {
		for j := 0; j < i; j++ {
			if nums[j] < nums[i] {
				if length[j]+1 > length[i] {
					length[i] = length[j] + 1
					count[i] = 0
				}
				if length[j]+1 == length[i] {
					count[i] += count[j]
				}
			}
		}
	}

	maxLength := 0
	result := 0

	for _, l := range length {
		if l > maxLength {
			maxLength = l
		}
	}

	for i := 0; i < len(nums); i++ {
		if length[i] == maxLength {
			result += count[i]
		}
	}

	return result
}
