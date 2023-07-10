package main

// https://leetcode.com/problems/minimum-depth-of-binary-tree/description/
func minDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}
	lev := []*TreeNode{root}
	d := 0
	for len(lev) > 0 {
		d++
		newLev := []*TreeNode{}

		for _, n := range lev {
			if n == nil {
				continue
			}

			if n.Left == nil && n.Right == nil {
				return d
			}

			newLev = append(newLev, n.Left, n.Right)
		}

		lev = newLev
	}
	return d
}
