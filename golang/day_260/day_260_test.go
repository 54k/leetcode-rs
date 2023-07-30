package day260

// https://leetcode.com/problems/populating-next-right-pointers-in-each-node-ii/description/
type Node struct {
	Val   int
	Left  *Node
	Right *Node
	Next  *Node
}

func connect(root *Node) *Node {
	if root == nil {
		return root
	}

	leftmost := root
	curr := leftmost
	var prev *Node

	processChild := func(childNode *Node) {
		if childNode != nil {
			if prev != nil {
				prev.Next = childNode
			} else {
				leftmost = childNode
			}
			prev = childNode
		}
	}

	for leftmost != nil {
		prev = nil
		curr = leftmost
		leftmost = nil

		for curr != nil {
			processChild(curr.Left)
			processChild(curr.Right)
			curr = curr.Next
		}
	}

	return root
}
