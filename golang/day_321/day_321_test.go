package day321

// https://leetcode.com/problems/monotonic-array/description
func isMonotonicStreamCompare(nums []int) bool {
	store := 0
	for i := 0; i < len(nums)-1; i++ {
		c := 0
		if nums[i] > nums[i+1] {
			c = 1
		} else if nums[i] < nums[i+1] {
			c = -1
		}

		if c != 0 {
			if c != store && store != 0 {
				return false
			}
			store = c
		}
	}
	return true
}

func isMonotonicSimple(nums []int) bool {
	inc := true
	dec := true
	for i := 0; i < len(nums)-1; i++ {
		if nums[i] < nums[i+1] {
			dec = false
		} else if nums[i] > nums[i+1] {
			inc = false
		}
	}
	return inc || dec
}
