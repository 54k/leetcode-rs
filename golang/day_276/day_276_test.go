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
	var check func(*TreeNode, int, int) bool
	check = func(root *TreeNode, left int, right int) bool {
		if root == nil {
			return true
		}
		if root.Val >= left || root.Val <= right {
			return false
		}
		return check(root.Left, root.Val, right) && check(root.Right, left, root.Val)
	}

	return check(root, (1 << 32), -(1 << 32))
}
