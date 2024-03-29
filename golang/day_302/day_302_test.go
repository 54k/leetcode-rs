package day302

// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func buildTreePreorder(preorder []int, inorder []int) *TreeNode {
	inorderIdx := map[int]int{}
	for i, n := range inorder {
		inorderIdx[n] = i
	}
	preorderIdx := 0
	var construct func(int, int) *TreeNode
	construct = func(left int, right int) *TreeNode {
		if left > right {
			return nil
		}
		val := preorder[preorderIdx]
		root := &TreeNode{val, nil, nil}
		preorderIdx++
		root.Left = construct(left, inorderIdx[val]-1)
		root.Right = construct(inorderIdx[val]+1, right)
		return root
	}

	return construct(0, len(inorder)-1)
}

// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
func buildTreePostorder(inorder []int, postorder []int) *TreeNode {
	inorderIdx := map[int]int{}
	for i, n := range inorder {
		inorderIdx[n] = i
	}
	postorderIdx := len(postorder) - 1
	var construct func(int, int) *TreeNode
	construct = func(left int, right int) *TreeNode {
		if left > right {
			return nil
		}
		val := postorder[postorderIdx]
		root := &TreeNode{val, nil, nil}
		postorderIdx--
		root.Right = construct(inorderIdx[val]+1, right)
		root.Left = construct(left, inorderIdx[val]-1)
		return root
	}
	return construct(0, len(inorder)-1)
}

// https://leetcode.com/problems/ternary-expression-parser/description/
func parseTernary(expression string) string {
	i := 0
	for i < len(expression) {
		if expression[i] != 'T' && expression[i] != 'F' || i == len(expression)-1 || expression[i+1] == ':' {
			break
		}
		if expression[i] == 'T' {
			i += 2
		} else {
			count := 1
			i += 2
			for ; count != 0; i++ {
				if expression[i] == ':' {
					count--
				} else if expression[i] == '?' {
					count++
				}
			}
		}
	}
	return expression[i : i+1]
}
