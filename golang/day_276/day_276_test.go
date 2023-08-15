package day276

// https://leetcode.com/problems/partition-list/description/
type ListNode struct {
	Val  int
	Next *ListNode
}

func partition(head *ListNode, x int) *ListNode {
	beforeHead := &ListNode{0, nil}
	beforeTail := beforeHead

	afterHead := &ListNode{0, nil}
	afterTail := afterHead

	for head != nil {
		if head.Val < x {
			beforeTail.Next = head
			beforeTail = beforeTail.Next
		} else {
			afterTail.Next = head
			afterTail = afterTail.Next
		}

		head = head.Next
	}
	afterTail.Next = nil
	beforeTail.Next = afterHead.Next

	return beforeHead.Next
}
