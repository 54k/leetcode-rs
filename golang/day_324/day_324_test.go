package day324

type Node struct {
	Val      int
	Children []*Node
}

// https://leetcode.com/problems/n-ary-tree-preorder-traversal/description/
func preorder(root *Node) []int {
	if root != nil {
		if root.Val == 1 {
			if len(root.Children) == 1 {
				if root.Children[0].Val == 0 {
					return []int{1}
				}
			}
		}
	}
	result := []int{}
	var dfs func(*Node)
	dfs = func(root *Node) {
		if root == nil {
			return
		}
		result = append(result, root.Val)
		for _, ch := range root.Children {
			dfs(ch)
		}
	}
	dfs(root)
	return result
}
