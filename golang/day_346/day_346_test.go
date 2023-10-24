package day346

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// https://leetcode.com/problems/find-largest-value-in-each-tree-row/description
func largestValues(root *TreeNode) []int {
	ans := []int{}
	var dfs func(*TreeNode, int)

	dfs = func(root *TreeNode, depth int) {
		if root == nil {
			return
		}

		if len(ans) == depth {
			ans = append(ans, root.Val)
		}

		if ans[depth] < root.Val {
			ans[depth] = root.Val
		}

		dfs(root.Left, depth+1)
		dfs(root.Right, depth+1)
	}

	dfs(root, 0)
	return ans
}
