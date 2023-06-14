package main

func getMinimumDifference(root *TreeNode) int {
	diff := 1000_000_000
	var prev *TreeNode

	abs := func(a, b int) int {
		if a < b {
			return b - a
		}
		return a - b
	}

	var md func(root *TreeNode)
	md = func(root *TreeNode) {
		if root != nil {
			md(root.Left)
			if prev != nil {
				if abs(prev.Val, root.Val) < diff {
					diff = abs(prev.Val, root.Val)
				}
			}
			prev = root
			md(root.Right)
		}
	}

	md(root)
	return diff
}
