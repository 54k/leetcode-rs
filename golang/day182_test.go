package main

import "testing"

type ListNode struct {
	Val  int
	Next *ListNode
}

func swapPairs1(head *ListNode) *ListNode {
	var f func(head *ListNode) *ListNode

	f = func(head *ListNode) *ListNode {
		node := head
		if node == nil {
			return node
		}

		for i := 0; i < 1; i++ {
			if node.Next != nil {
				node = node.Next
			} else {
				return head
			}
		}

		n := node.Next
		node.Next = nil
		tail := f(n)

		for n := head; n != nil; n = n.Next {
			tail = &ListNode{
				Val:  n.Val,
				Next: tail,
			}
		}

		return tail
	}

	return f(head)
}

func swapPairs2(head *ListNode) *ListNode {
	var f func(head *ListNode) *ListNode

	f = func(head *ListNode) *ListNode {
		if head == nil || head.Next == nil {
			return head
		}

		cur := head
		next := head.Next

		t := f(next.Next)
		cur.Next = t
		next.Next = head
		return next
	}

	return f(head)
}

func swapPairs3(head *ListNode) *ListNode {
	dummy := &ListNode{
		Val:  0,
		Next: head,
	}

	prevNode := dummy
	for head != nil && head.Next != nil {
		first := head
		second := head.Next

		prevNode.Next = second
		first.Next = second.Next
		second.Next = first

		prevNode = first
		head = first.Next
	}

	return dummy.Next
}

func reverseListIt(head *ListNode) *ListNode {
	var prev *ListNode
	for head != nil {
		next := head.Next
		head.Next = prev
		prev = head
		head = next
	}

	return prev
}

func reverseListRec(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}

	p := reverseListRec(head.Next)
	head.Next.Next = head
	head.Next = nil
	return p
}

func TestSwapPairs(t *testing.T) {
	list := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 3,
				Next: &ListNode{
					Val:  4,
					Next: nil,
				},
			},
		},
	}

	println(swapPairs1(list))

}
