package day356

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// https://leetcode.com/problems/add-one-row-to-tree/description/
func addOneRow(root *TreeNode, val int, depth int) *TreeNode {
	var dfs func(*TreeNode, int)
	dfs = func(node *TreeNode, cur int) {
		if node == nil {
			return
		} else if cur == depth-1 {
			t := node.Left
			left := &TreeNode{val, nil, nil}
			left.Left = t
			node.Left = left
			t = node.Right
			right := &TreeNode{val, nil, nil}
			right.Right = t
			node.Right = right
		} else {
			dfs(node.Left, cur+1)
			dfs(node.Right, cur+1)
		}
	}

	if depth == 1 {
		return &TreeNode{val, root, nil}
	}
	dfs(root, 1)
	return root
}
