package day257

// https://leetcode.com/problems/maximum-running-time-of-n-computers/description/
func maxRunTime(n int, batteries []int) int64 {
	sumPower := 0
	for _, pow := range batteries {
		sumPower += pow
	}

	left, right := int64(1), int64(sumPower/n)

	for left < right {
		target := right - (right-left)/2
		extra := int64(0)

		for _, pow := range batteries {
			pow := int64(pow)
			if pow < target {
				extra += pow
			} else {
				extra += target
			}
		}

		if extra >= int64(n)*target {
			left = target
		} else {
			right = target - 1
		}
	}

	return left
}

// https://leetcode.com/problems/binary-tree-inorder-traversal/description/
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func inorderTraversalMorris(root *TreeNode) []int {
	res := []int{}
	curr := root
	var pre *TreeNode

	for curr != nil {
		if curr.Left == nil {
			res = append(res, curr.Val)
			curr = curr.Right
		} else {
			pre = curr.Left
			for pre.Right != nil {
				pre = pre.Right
			}

			pre.Right = curr
			temp := curr
			curr = curr.Left
			temp.Left = nil
		}
	}

	return res
}

// https://leetcode.com/problems/count-univalue-subtrees/description/
func countUnivalSubtrees(root *TreeNode) int {
	count := 0

	var dfs func(*TreeNode) bool
	dfs = func(node *TreeNode) bool {
		if node == nil {
			return true
		}

		left := dfs(node.Left)
		right := dfs(node.Right)

		if left && right {
			if node.Left != nil && node.Left.Val != node.Val {
				return false
			}
			if node.Right != nil && node.Right.Val != node.Val {
				return false
			}
			count++
			return true
		}

		return false
	}

	dfs(root)
	return count
}
