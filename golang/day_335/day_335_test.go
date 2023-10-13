package day335

// https://leetcode.com/problems/binary-tree-preorder-traversal/description/
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func preorderTraversalMorris(root *TreeNode) []int {
	ans := []int{}
	curr := root
	var last *TreeNode
	for curr != nil {
		if curr.Left == nil {
			ans = append(ans, curr.Val)
			curr = curr.Right
		} else {
			last = curr.Left
			for last.Right != nil && last.Right != curr {
				last = last.Right
			}

			if last.Right == nil {
				ans = append(ans, curr.Val)
				last.Right = curr
				curr = curr.Left
			} else {
				last.Right = nil
				curr = curr.Right
			}
		}
	}
	return ans
}
