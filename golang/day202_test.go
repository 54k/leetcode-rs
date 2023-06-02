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
