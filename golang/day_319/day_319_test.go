package day319

import "sort"

// https://leetcode.com/problems/3sum-closest/description/
func threeSumClosestTwoPointers(nums []int, target int) int {
	abs := func(a int) int {
		if a < 0 {
			return -a
		}
		return a
	}
	sort.Ints(nums)
	diff := 1 << 30
	for i := 0; i < len(nums); i++ {
		lo := i + 1
		hi := len(nums) - 1

		for lo < hi {
			sum := nums[i] + nums[lo] + nums[hi]
			if abs(target-sum) < abs(diff) {
				diff = target - sum
			}

			if sum < target {
				lo++
			} else {
				hi--
			}
		}
	}
	return target - diff
}

func threeSumClosestBinarySearch(nums []int, target int) int {
	abs := func(a int) int {
		if a < 0 {
			return -a
		}
		return a
	}
	binSearch := func(start, complement int) int {
		lo := start + 1
		hi := len(nums) - 1
		for lo <= hi {
			mid := (lo + hi) / 2
			if nums[mid] == complement {
				return mid
			} else if nums[mid] < complement {
				lo = mid + 1
			} else {
				hi = mid - 1
			}
		}
		return lo
	}

	diff := 1 << 30
	sort.Ints(nums)
	for i := 0; i < len(nums); i++ {
		for j := i + 1; j < len(nums)-1; j++ {
			complement := target - nums[i] - nums[j]
			idx := binSearch(j, complement)
			hi := idx
			lo := hi - 1

			if hi < len(nums) && abs(complement-nums[hi]) < abs(diff) {
				diff = complement - nums[hi]
			}

			if lo > j && abs(complement-nums[lo]) < abs(diff) {
				diff = complement - nums[lo]
			}
		}
	}
	return target - diff
}
