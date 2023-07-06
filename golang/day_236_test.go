package main

// https://leetcode.com/problems/minimum-size-subarray-sum/description/
func minSubArrayLen(target int, nums []int) int {
	ans, left, sum := 1<<31, 0, 0

	for right := 0; right < len(nums); right++ {
		sum += nums[right]
		for sum >= target {
			if right-left+1 < ans {
				ans = right - left + 1
			}
			sum -= nums[left]
			left++
		}
	}

	if ans == 1<<31 {
		return 0
	}
	return ans
}
