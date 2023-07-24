package day_254

import "math/rand"

func topKFrequent(nums []int, k int) []int {
	count := map[int]int{}
	for _, num := range nums {
		count[num]++
	}
	unique := []int{}
	for k, _ := range count {
		unique = append(unique, k)
	}

	swap := func(left, right int) {
		tmp := unique[left]
		unique[left] = unique[right]
		unique[right] = tmp
	}

	partition := func(left int, right int, pivot_idx int) int {
		pivot_freq := count[unique[pivot_idx]]
		swap(pivot_idx, right)
		store_idx := left

		for i := left; i < right; i++ {
			if count[unique[i]] < pivot_freq {
				swap(store_idx, i)
				store_idx++
			}
		}

		swap(store_idx, right)
		return store_idx
	}

	var quickselect func(int, int, int)
	quickselect = func(left int, right int, k_smallest int) {
		if left == right {
			return
		}
		pivot_idx := left + rand.Intn(right-left)
		pivot_idx = partition(left, right, pivot_idx)

		if k_smallest == pivot_idx {
			return
		} else if k_smallest < pivot_idx {
			quickselect(left, pivot_idx-1, k_smallest)
		} else {
			quickselect(pivot_idx+1, right, k_smallest)
		}
	}

	n := len(unique)

	quickselect(0, n-1, n-k)

	return unique[n-k : n]
}
