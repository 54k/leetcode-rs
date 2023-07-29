package day259

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/description/
func buildTreeInorderPostorder(inorder []int, postorder []int) *TreeNode {
	postIdx := len(postorder) - 1

	idxMap := map[int]int{}
	for i, v := range inorder {
		idxMap[v] = i
	}

	var helper func(int, int) *TreeNode
	helper = func(left int, right int) *TreeNode {
		if left > right {
			return nil
		}

		rootVal := postorder[postIdx]
		postIdx--
		root := &TreeNode{rootVal, nil, nil}
		root.Right = helper(idxMap[rootVal]+1, right)
		root.Left = helper(left, idxMap[rootVal]-1)
		return root
	}

	return helper(0, len(inorder)-1)
}

// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/description/
func buildTreePreorderInorder(preorder []int, inorder []int) *TreeNode {
	preorderIdx := 0

	inorderIndexMap := map[int]int{}
	for i := 0; i < len(inorder); i++ {
		inorderIndexMap[inorder[i]] = i
	}

	var arrayToTree func(int, int) *TreeNode
	arrayToTree = func(left int, right int) *TreeNode {
		if left > right {
			return nil
		}

		rootValue := preorder[preorderIdx]
		preorderIdx++
		root := &TreeNode{rootValue, nil, nil}
		root.Left = arrayToTree(left, inorderIndexMap[rootValue]-1)
		root.Right = arrayToTree(inorderIndexMap[rootValue]+1, right)
		return root
	}

	return arrayToTree(0, len(preorder)-1)
}
