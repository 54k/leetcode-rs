package day277

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func inorderSuccessor(root *TreeNode, p *TreeNode) *TreeNode {
	if p.Right != nil {
		left := p.Right
		for left.Left != nil {
			left = left.Left
		}
		return left
	} else {
		stack := []*TreeNode{}
		node := root

		for len(stack) > 0 || node != nil {
			for node != nil {
				stack = append(stack, node)
				node = node.Left
			}

			top := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			if top == p {
				if len(stack) == 0 {
					return nil
				}
				return stack[len(stack)-1]
			}

			node = top.Right
		}

		return nil
	}
}

func inorderSuccessorBST(root *TreeNode, p *TreeNode) *TreeNode {
	var succ *TreeNode
	node := root
	for node != nil {
		if p.Val >= node.Val {
			node = node.Right
		} else {
			succ = node
			node = node.Left
		}
	}
	return succ
}

// https://leetcode.com/problems/inorder-successor-in-bst-ii/description/
type Node struct {
	Val    int
	Left   *Node
	Right  *Node
	Parent *Node
}

func inorderSuccessorWithParent(node *Node) *Node {
	if node.Right != nil {
		node := node.Right
		for node.Left != nil {
			node = node.Left
		}
		return node
	}

	for node.Parent != nil && node == node.Parent.Right {
		node = node.Parent
	}
	return node.Parent
}

// https://leetcode.com/problems/binary-search-tree-iterator/description
type BSTIterator struct {
	stack []*TreeNode
	curr  *TreeNode
}

func Constructor(root *TreeNode) BSTIterator {
	return BSTIterator{[]*TreeNode{}, root}
}

func (this *BSTIterator) Next() int {
	curr := this.curr
	stack := this.stack

	if len(stack) > 0 || curr != nil {
		for curr != nil {
			stack = append(stack, curr)
			curr = curr.Left
		}

		top := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		this.curr = top.Right
		this.stack = stack
		return top.Val
	}

	panic("oops")
}

func (this *BSTIterator) HasNext() bool {
	return len(this.stack) > 0 || this.curr != nil
}
