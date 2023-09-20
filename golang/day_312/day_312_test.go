package day312

// https://leetcode.com/problems/maximum-size-subarray-sum-equals-k/
func maxSubArrayLen(nums []int, k int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	indices := map[int]int{}
	maxSize := 0
	current := 0

	for right := 0; right < len(nums); right++ {
		current += nums[right]
		if current == k {
			maxSize = right + 1
		}
		if _, ok := indices[current-k]; ok {
			maxSize = max(maxSize, right-indices[current-k])
		}
		if _, ok := indices[current]; !ok {
			indices[current] = right
		}
	}
	return maxSize
}
