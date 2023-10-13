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

// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together/description/
func minSwaps_SlidingWindow(data []int) int {
	ones := 0
	for _, n := range data {
		if n == 1 {
			ones++
		}
	}

	count := 0
	ans := 1 << 30
	j := 0

	for i := 0; i < len(data); i++ {
		if data[i] == 1 {
			count++
		}

		if i >= ones {
			if data[j] == 1 {
				count--
			}
			j++
		}

		if ans > ones-count {
			ans = ones - count
		}
	}
	return ans
}
