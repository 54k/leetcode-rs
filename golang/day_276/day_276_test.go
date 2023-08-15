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

// https://leetcode.com/problems/validate-binary-search-tree/description/
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isValidBST(root *TreeNode) bool {
	var isValid func(*TreeNode, int, int) bool
	isValid = func(root *TreeNode, leftBound int, rightBound int) bool {
		if root == nil {
			return true
		}

		if root.Val <= leftBound || root.Val >= rightBound {
			return false
		}
		return isValid(root.Right, root.Val, rightBound) && isValid(root.Left, leftBound, root.Val)
	}

	return isValid(root, -(1 << 32), (1 << 32))
}
