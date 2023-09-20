package day312

import (
	"fmt"
	"testing"
)

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

// https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero/
func minOperationsIndirectly(nums []int, x int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	sum := 0
	for _, num := range nums {
		sum += num
	}

	maxSize := -1
	current := 0

	left := 0
	for right := 0; right < len(nums); right++ {
		current += nums[right]

		for sum-x < current && left <= right {
			current -= nums[left]
			left++
		}
		if sum-x == current {
			maxSize = max(maxSize, right-left+1)
		}
	}

	if maxSize == -1 {
		return -1
	}
	return len(nums) - maxSize
}

func minOperationsDirectly(nums []int, x int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	current := 0
	for _, n := range nums {
		current += n
	}

	ans := 1 << 31
	left := 0

	for right := 0; right < len(nums); right++ {
		current -= nums[right]

		for current < x && left <= right {
			current += nums[left]
			left++
		}

		if current == x {
			ans = min(ans, (len(nums)-1-right)+left)
		}
	}

	if ans == 1<<31 {
		return -1
	}
	return ans
}

func TestMinOperations(t *testing.T) {
	res := minOperationsDirectly([]int{3, 2, 20, 1, 1, 3}, 10)
	fmt.Println(res) // 5
}

// https://leetcode.com/problems/find-the-duplicate-number/
func findDuplicateBinarySearch(nums []int) int {
	left := 1
	right := len(nums)
	ans := -1
	for left <= right {
		mid := (left + right) / 2
		count := 0

		for _, num := range nums {
			if num <= mid {
				count++
			}
		}

		if count > mid {
			right = mid - 1
			ans = mid
		} else {
			left = mid + 1
		}
	}
	return ans
}

func findDuplicateSumOfSetBits(nums []int) int {
	max := 0
	for _, num := range nums {
		if num > max {
			max = num
		}
	}
	maxBitPos := func(num int) int {
		ans := 0
		for num > 0 {
			ans++
			num >>= 1
		}
		return ans
	}

	maxBit := maxBitPos(max)
	duplicate := 0
	for i := 0; i < maxBit; i++ {
		mask := 1 << i
		baseCount := 0
		freqOfBit := 0

		for i, num := range nums {
			if mask&i > 1 {
				baseCount++
			}
			if mask&num > 0 {
				freqOfBit++
			}
		}

		if freqOfBit > baseCount {
			duplicate |= mask
		}
	}

	return duplicate
}
