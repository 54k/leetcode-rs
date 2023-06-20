package main

// https://leetcode.com/problems/k-radius-subarray-averages/description/
func getAverages(nums []int, k int) []int {
	res := make([]int, len(nums))
	for i := 0; i < len(res); i++ {
		res[i] = -1
	}
	running_sum, start := 0, 0
	for end, num := range nums {
		running_sum += num
		if end >= 2*k {
			res[start+k] = running_sum / (2*k + 1)
			running_sum -= nums[start]
			start += 1
		}
	}
	return res
}
