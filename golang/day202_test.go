package main

// https://leetcode.com/problems/maximum-subarray/description/
func maxSubArray(nums []int) int {
	current, best := 0, -1000_000_000
	for _, num := range nums {
		if num+current < num {
			current = num
		} else {
			current += num
		}
		if current > best {
			best = current
		}
	}
	return best
}

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/
func maxProfit1(prices []int) int {
	min, best := 1000_000_000, 0
	for _, val := range prices {
		if min > val {
			min = val
		}
		if val-min > best {
			best = val - min
		}
	}
	return best
}

// https://leetcode.com/problems/maximum-sum-circular-subarray/description/
func maxSubarraySumCircular(nums []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	n := len(nums)
	rightMax := make([]int, n)
	rightMax[n-1] = nums[n-1]

	for suffixSum, i := nums[n-1], n-2; i >= 0; i-- {
		suffixSum += nums[i]
		rightMax[i] = suffixSum
		if rightMax[i+1] > rightMax[i] {
			rightMax[i] = rightMax[i+1]
		}
	}

	maxSum := nums[0]
	specialSum := nums[0]

	for i, prefixSum, curMax := 0, 0, 0; i < n; i++ {
		curMax = max(curMax, 0) + nums[i]
		maxSum = max(maxSum, curMax)
		prefixSum += nums[i]
		if i+1 < n {
			specialSum = max(specialSum, prefixSum+rightMax[i+1])
		}
	}
	return max(maxSum, specialSum)
}
