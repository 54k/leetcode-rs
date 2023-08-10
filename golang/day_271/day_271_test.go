package day271

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
