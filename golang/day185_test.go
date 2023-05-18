package main

// https://leetcode.com/problems/swap-nodes-in-pairs/
func swapRec(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}

	firstNode := head
	secondNode := head.Next

	firstNode.Next = swapPairs(secondNode.Next)
	secondNode.Next = firstNode

	return secondNode
}

func swapIter(head *ListNode) *ListNode {
	dummy := &ListNode{
		Val:  0,
		Next: head,
	}
	prev := dummy

	for head != nil && head.Next != nil {
		first := head
		second := head.Next

		prev.Next = second
		first.Next = second.Next
		second.Next = first

		prev = first
		head = first.Next
	}
	return dummy.Next
}

func swapPairs(head *ListNode) *ListNode {
	return swapIter(head)
}

type ImmutableListNode struct {
}

func (this *ImmutableListNode) getNext() *ImmutableListNode {
	return &ImmutableListNode{}
}

func (this *ImmutableListNode) printValue() {
	// print the value of this node.
}

func printLinkedListInReverse(head *ImmutableListNode) {
	if head == nil {
		return
	}
	printLinkedListInReverse(head.getNext())
	head.printValue()
}
