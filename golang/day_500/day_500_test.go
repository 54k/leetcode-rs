package day500

import "math"

// https://leetcode.com/problems/subarray-product-less-than-k/description/
func numSubarrayProductLessThanK(nums []int, k int) int {
	if k == 0 {
		return 0
	}

	logK := math.Log(float64(k))
	m := len(nums) + 1
	logPrefixSum := make([]float64, m)

	for i := 0; i < len(nums); i++ {
		logPrefixSum[i+1] = logPrefixSum[i] + math.Log(float64(nums[i]))
	}

	totalCount := 0

	for currIdx := 0; currIdx < m; currIdx++ {
		low, hi := currIdx+1, m
		for low < hi {
			mid := low + (hi-low)/2
			if logPrefixSum[mid] >= logPrefixSum[currIdx]+logK-1e-9 {
				hi = mid
			} else {
				low = mid + 1
			}
		}
		totalCount += low - currIdx - 1
	}

	return totalCount
}
