package day271

// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/
func search(nums []int, target int) bool {
	existsInFirst := func(start int, el int) bool {
		return nums[start] <= el
	}
	isBinaryHelpful := func(left int, el int) bool {
		return nums[left] != el
	}
	left, right := 0, len(nums)-1
	for left <= right {
		mid := left + (right-left)/2
		if nums[mid] == target {
			return true
		}
		if !isBinaryHelpful(left, nums[mid]) {
			left++
			continue
		}
		pivotArray := existsInFirst(left, nums[mid])
		targetArray := existsInFirst(left, target)

		if pivotArray != targetArray {
			if pivotArray {
				left = mid + 1
			} else {
				right = mid - 1
			}
		} else {
			if nums[mid] < target {
				left = mid + 1
			} else {
				right = mid - 1
			}
		}
	}
	return false
}

// https://leetcode.com/problems/sort-colors/description/
func sortColors(nums []int) {
	p0 := 0
	curr := 0
	p2 := len(nums) - 1

	var tmp int
	for curr <= p2 {
		if nums[curr] == 0 {
			tmp = nums[p0]
			nums[p0] = nums[curr]
			nums[curr] = tmp
			p0++
			curr++
		} else if nums[curr] == 2 {
			tmp = nums[curr]
			nums[curr] = nums[p2]
			nums[p2] = tmp
			p2--
		} else {
			curr++
		}
	}
}
