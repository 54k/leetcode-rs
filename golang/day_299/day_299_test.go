package day299

type ListNode struct {
	Val  int
	Next *ListNode
}

// https://leetcode.com/problems/reverse-linked-list-ii/description/
func reverseBetweenRec(head *ListNode, m int, n int) *ListNode {
	left := head
	stop := false

	var rec func(*ListNode, int, int)
	rec = func(right *ListNode, m int, n int) {
		if n == 1 {
			return
		}
		right = right.Next

		if m > 1 {
			left = left.Next
		}

		rec(right, m-1, n-1)

		if left == right || right.Next == left {
			stop = true
		}

		if !stop {
			t := left.Val
			left.Val = right.Val
			right.Val = t
			left = left.Next
		}
	}

	rec(head, m, n)
	return head
}

func reverseBetweenIter(head *ListNode, left int, right int) *ListNode {
	if head == nil {
		return nil
	}

	cur := head
	var prev *ListNode
	for left > 1 {
		prev = cur
		cur = cur.Next
		left--
		right--
	}

	con := prev
	tail := cur

	var third *ListNode
	for right > 0 {
		third = cur.Next
		cur.Next = prev
		prev = cur
		cur = third
		right--
	}

	if con != nil {
		con.Next = prev
	} else {
		head = prev
	}
	tail.Next = cur
	return head
}
