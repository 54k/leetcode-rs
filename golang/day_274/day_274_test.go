package day274

import (
	"fmt"
	"testing"
)

// https://leetcode.com/problems/check-if-there-is-a-valid-partition-for-the-array/description/
func validPartition(nums []int) bool {
	n := len(nums)
	dp := make([]int, n+1)
	dp[0] = 1

	for i := 0; i < n; i++ {
		dpIndex := i + 1

		if i > 0 && nums[i] == nums[i-1] {
			dp[dpIndex] |= dp[dpIndex-2]
		}

		if i > 1 && nums[i] == nums[i-1] && nums[i] == nums[i-2] {
			dp[dpIndex] |= dp[dpIndex-3]
		}

		if i > 1 && nums[i] == nums[i-1]+1 && nums[i-1] == nums[i-2]+1 {
			dp[dpIndex] |= dp[dpIndex-3]
		}
	}

	return dp[n] == 1
}

type ListNode struct {
	Val  int
	Next *ListNode
}

func doubleIt(head *ListNode) *ListNode {
	rev := func(h *ListNode) *ListNode {
		var newHead *ListNode
		for h != nil {
			nxt := h.Next
			h.Next = newHead
			newHead = h
			h = nxt
		}
		return newHead
	}

	head = rev(head)
	ret := head
	sum := 0
	for head != nil {
		sum += head.Val * 2
		head.Val = sum % 10
		sum /= 10
		head = head.Next
	}
	ret = rev(ret)
	if sum > 0 {
		ret = &ListNode{sum, ret}
	}
	return ret
}

func TestDoubleIT(t *testing.T) {
	res := doubleIt(&ListNode{9, &ListNode{9, &ListNode{9, nil}}})
	fmt.Println(res)
}
