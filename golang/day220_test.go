package main

import "fmt"

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

// https://leetcode.com/problems/bitwise-and-of-numbers-range/description/
func rangeBitwiseAnd(m, n int) int {
	fmt.Printf("m %08b\n", m)
	fmt.Printf("n %08b\n", n)
	for m < n {
		n = n & (n - 1)
		fmt.Printf("%08b\n", n)
	}
	return m & n
}

// https://leetcode.com/problems/counting-bits/description/
func countBits(n int) []int {
	popCount := func(x int) int {
		var cnt int
		for x != 0 {
			x &= x - 1
			cnt++
		}
		return cnt
	}
	ans := make([]int, n+1)
	for i := 0; i <= n; i++ {
		ans[i] = popCount(i)
	}
	return ans
}

func countBits2(n int) []int {
	ans := make([]int, n+1)
	for x := 1; x <= n; x++ {
		ans[x] = ans[x&(x-1)] + 1
	}
	return ans
}
