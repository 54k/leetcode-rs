package main

// https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list/description/
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func getMid(head *ListNode) *ListNode {
	slow, fast := head, head
	for fast != nil && fast.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next
	}
	return slow
}

func reverse(head *ListNode) *ListNode {
	var prev *ListNode
	for head != nil {
		next := head.Next
		head.Next = prev
		prev = head
		head = next
	}
	return prev
}

func pairSum(head *ListNode) int {
	ans := 0
	mid := reverse(getMid(head))

	for mid != nil {
		if head.Val+mid.Val >= ans {
			ans = head.Val + mid.Val
		}
		head = head.Next
		mid = mid.Next
	}

	return ans
}

// https://leetcode.com/problems/delete-node-in-a-bst/description/
func succ(root *TreeNode) int {
	root = root.Right
	for root.Left != nil {
		root = root.Left
	}
	return root.Val
}

func pred(root *TreeNode) int {
	root = root.Left
	for root.Right != nil {
		root = root.Right
	}
	return root.Val
}

func deleteNode(root *TreeNode, key int) *TreeNode {
	if root == nil {
		return nil
	}

	if key > root.Val {
		root.Right = deleteNode(root.Right, key)
	} else if key < root.Val {
		root.Left = deleteNode(root.Left, key)
	} else {
		if root.Left == nil && root.Right == nil {
			root = nil
		} else if root.Right != nil {
			root.Val = succ(root)
			root.Right = deleteNode(root.Right, root.Val)
		} else {
			root.Val = pred(root)
			root.Left = deleteNode(root.Left, root.Val)
		}
	}
	return root
}
