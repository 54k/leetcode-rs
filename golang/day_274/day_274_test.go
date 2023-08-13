package day274

// https://leetcode.com/problems/check-if-there-is-a-valid-partition-for-the-array/description/
func validPartition(nums []int) bool {
	n := len(nums)
	dp := make([]int, n+1)
	dp[0] = 1

	for i := 0; i < n; i++ {
		dpIndex := i + 1

		if i > 0 && nums[i] == nums[i-1] {
			dp[dpIndex] |= dp[dpIndex-2]
		}

		if i > 1 && nums[i] == nums[i-1] && nums[i] == nums[i-2] {
			dp[dpIndex] |= dp[dpIndex-3]
		}

		if i > 1 && nums[i] == nums[i-1]+1 && nums[i-1] == nums[i-2]+1 {
			dp[dpIndex] |= dp[dpIndex-3]
		}
	}

	return dp[n] == 1
}
