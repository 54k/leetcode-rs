package main

// https://leetcode.com/problems/swapping-nodes-in-a-linked-list/description/
func swapNodes(head *ListNode, k int) *ListNode {
	len := 0
	var fronNode *ListNode = nil
	var endNode *ListNode = nil
	currentNode := head

	for currentNode != nil {
		len++

		if endNode != nil {
			endNode = endNode.Next
		}

		if len == k {
			fronNode = currentNode
			endNode = head
		}

		currentNode = currentNode.Next
	}

	temp := fronNode.Val
	fronNode.Val = endNode.Val
	endNode.Val = temp
	return head
}
