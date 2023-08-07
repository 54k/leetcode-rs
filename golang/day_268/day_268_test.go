package day268

// https://leetcode.com/problems/search-a-2d-matrix/description/
func searchMatrix(matrix [][]int, target int) bool {
	m, n := len(matrix), len(matrix[0])
	left, right := 0, m*n-1
	for left <= right {
		mid := left + (right-left)/2
		r, c := mid/n, mid%n
		if matrix[r][c] == target {
			return true
		} else if matrix[r][c] < target {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}
	return false
}

// https://leetcode.com/problems/flatten-a-multilevel-doubly-linked-list/description/
type NodeWithChild struct {
	Val   int
	Prev  *NodeWithChild
	Next  *NodeWithChild
	Child *NodeWithChild
}

func flattenDfs(root *NodeWithChild) *NodeWithChild {
	if root == nil {
		return root
	}
	dummyHead := &NodeWithChild{0, nil, root, nil}
	var flattenDfs func(*NodeWithChild, *NodeWithChild) *NodeWithChild
	flattenDfs = func(prev *NodeWithChild, curr *NodeWithChild) *NodeWithChild {
		if curr == nil {
			return prev
		}
		curr.Prev = prev
		prev.Next = curr

		tempNext := curr.Next

		tail := flattenDfs(curr, curr.Child)
		curr.Child = nil

		return flattenDfs(tail, tempNext)
	}
	flattenDfs(dummyHead, root)
	dummyHead.Next.Prev = nil
	return dummyHead.Next
}

func flatten(head *NodeWithChild) *NodeWithChild {
	if head == nil {
		return head
	}
	dummy := &NodeWithChild{0, nil, head, nil}
	var curr *NodeWithChild
	prev := dummy

	stack := []*NodeWithChild{}
	stack = append(stack, head)

	for len(stack) > 0 {
		curr = stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		prev.Next = curr
		curr.Prev = prev

		if curr.Next != nil {
			stack = append(stack, curr.Next)
		}
		if curr.Child != nil {
			stack = append(stack, curr.Child)
			curr.Child = nil
		}
		prev = curr
	}

	dummy.Next.Prev = nil
	return dummy.Next
}

// https://leetcode.com/problems/insert-into-a-sorted-circular-linked-list/description/
type Node struct {
	Val  int
	Next *Node
}

func insert(head *Node, insertVal int) *Node {
	if head == nil {
		newNode := &Node{insertVal, nil}
		newNode.Next = newNode
		return newNode
	}
	prev := head
	curr := head.Next
	toInsert := false

	for {
		if prev.Val <= insertVal && insertVal <= curr.Val {
			toInsert = true
		} else if prev.Val > curr.Val {
			if insertVal >= prev.Val || insertVal <= curr.Val {
				toInsert = true
			}
		}
		if toInsert {
			prev.Next = &Node{insertVal, curr}
			return head
		}
		prev = curr
		curr = curr.Next

		if prev == head {
			break
		}
	}
	prev.Next = &Node{insertVal, curr}
	return head
}
