package day272

import "math/rand"

// https://leetcode.com/problems/coin-change-ii/description/
func change(amount int, coins []int) int {
	dp := make([]int, amount+1)
	dp[0] = 1
	for _, coin := range coins {
		for amt := 1; amt <= amount; amt++ {
			if amt >= coin {
				dp[amt] += dp[amt-coin]
			}
		}
	}
	return dp[amount]
}

// https://leetcode.com/problems/insertion-sort-list/
type ListNode struct {
	Val  int
	Next *ListNode
}

func insertionSortList(head *ListNode) *ListNode {
	dummy := &ListNode{0, nil}
	curr := head
	for curr != nil {
		prev := dummy

		for prev.Next != nil && prev.Next.Val <= curr.Val {
			prev = prev.Next
		}

		next := curr.Next
		curr.Next = prev.Next
		prev.Next = curr
		curr = next
	}
	return dummy.Next
}

// https://leetcode.com/problems/kth-largest-element-in-an-array/description/
func findKthLargest(nums []int, k int) int {
	var quickselect func([]int, int) int
	quickselect = func(nums []int, k int) int {
		pIdx := rand.Int() % len(nums)
		pivot := nums[pIdx]

		left := []int{}
		mid := []int{}
		right := []int{}

		for _, n := range nums {
			if n < pivot {
				right = append(right, n)
			} else if n == pivot {
				mid = append(mid, n)
			} else {
				left = append(left, n)
			}
		}

		if len(left) >= k {
			return quickselect(left, k)
		}
		if len(mid)+len(left) < k {
			return quickselect(right, k-len(mid)-len(left))
		}
		return pivot
	}
	return quickselect(nums, k)
}

// https://leetcode.com/problems/top-k-frequent-elements/description/
func topKFrequent(nums []int, k int) []int {
	freq := map[int]int{}
	for _, n := range nums {
		freq[n]++
	}
	n := len(freq)
	uniq := make([]int, n)
	i := 0
	for k := range freq {
		uniq[i] = k
		i++
	}

	swap := func(left int, right int) {
		tmp := uniq[left]
		uniq[left] = uniq[right]
		uniq[right] = tmp
	}

	partition := func(left int, right int, pivotIndex int) int {
		pivotFrequency := freq[uniq[pivotIndex]]
		swap(pivotIndex, right)
		storeIdx := left
		for i := left; i <= right; i++ {
			if freq[uniq[i]] < pivotFrequency {
				swap(storeIdx, i)
				storeIdx++
			}
		}
		swap(storeIdx, right)
		return storeIdx
	}

	var quickselect func(int, int, int)
	quickselect = func(left int, right int, k int) {
		if left == right {
			return
		}
		pivotIndex := left + rand.Intn(right-left)
		pivotIndex = partition(left, right, pivotIndex)

		if k == pivotIndex {
			return
		} else if k < pivotIndex {
			quickselect(left, pivotIndex-1, k)
		} else {
			quickselect(pivotIndex+1, right, k)
		}
	}

	quickselect(0, n-1, n-k)
	return uniq[n-k:]
}
