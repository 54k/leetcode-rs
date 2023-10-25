package day347

// https://leetcode.com/problems/maximum-subarray/description/
func maxSubArray(nums []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	var findBest func(int, int) int
	findBest = func(left, right int) int {
		if left > right {
			return -(1 << 30)
		}

		mid := (right-left)/2 + left
		curr := 0
		leftSum := 0
		for i := mid - 1; i >= left; i-- {
			curr += nums[i]
			leftSum = max(leftSum, curr)
		}
		curr = 0
		rightSum := 0
		for i := mid + 1; i <= right; i++ {
			curr += nums[i]
			rightSum = max(rightSum, curr)
		}

		bothSum := nums[mid] + leftSum + rightSum

		return max(bothSum, max(findBest(left, mid-1), findBest(mid+1, right)))
	}
	return findBest(0, len(nums)-1)
}
