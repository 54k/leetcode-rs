package day291

// https://leetcode.com/problems/minimum-replacements-to-sort-the-array/description/
func minimumReplacement(nums []int) int64 {
	nn := make([]int64, len(nums))
	for i := 0; i < len(nums); i++ {
		nn[i] = int64(nums[i])
	}
	ops := int64(0)
	for i := len(nums) - 2; i >= 0; i-- {
		if nn[i] < nn[i+1] {
			continue
		}
		buckets := int64(nn[i]+nn[i+1]-1) / int64(nn[i+1])
		ops += buckets - 1
		nn[i] /= buckets
	}
	return ops
}
