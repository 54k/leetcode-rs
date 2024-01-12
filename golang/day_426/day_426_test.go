package day426

// https://leetcode.com/problems/find-k-closest-elements/description/
func findClosestElements(arr []int, k int, x int) []int {
	var abs = func(a int) int {
		if a < 0 {
			return -a
		}
		return a
	}

	result := []int{}
	if len(arr) == k {
		for i := 0; i < k; i++ {
			result = append(result, arr[i])
		}
		return result
	}

	left, right := 0, len(arr)
	for left < right {
		mid := (left + right) / 2

		if arr[mid] < x {
			left = mid + 1
		} else {
			right = mid
		}
	}

	left -= 1
	right = left + 1

	for right-left-1 < k {
		if left == -1 {
			right += 1
			continue
		}

		if right == len(arr) || abs(arr[left]-x) <= abs(arr[right]-x) {
			left -= 1
		} else {
			right += 1
		}
	}

	for i := left + 1; i < right; i++ {
		result = append(result, arr[i])
	}

	return result
}
