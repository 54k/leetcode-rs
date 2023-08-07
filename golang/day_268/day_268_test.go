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

// https://leetcode.com/problems/copy-list-with-random-pointer/description/
type NodeRandom struct {
	Val    int
	Next   *NodeRandom
	Random *NodeRandom
}

func copyRandomList(head *NodeRandom) *NodeRandom {
	visited := map[*NodeRandom]*NodeRandom{}
	var copy func(*NodeRandom) *NodeRandom
	copy = func(head *NodeRandom) *NodeRandom {
		if head == nil {
			return head
		}
		if _, ok := visited[head]; ok {
			return visited[head]
		}
		node := &NodeRandom{head.Val, nil, nil}
		visited[head] = node
		node.Next = copy(head.Next)
		node.Random = copy(head.Random)
		return node
	}
	return copy(head)
}

// https://leetcode.com/problems/rotate-list/description/
type ListNode struct {
	Val  int
	Next *ListNode
}

func rotateRight(head *ListNode, k int) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}

	oldTail := head
	var n int
	for n = 1; oldTail.Next != nil; n++ {
		oldTail = oldTail.Next
	}
	oldTail.Next = head

	newTail := head
	for i := 0; i < n-k%n-1; i++ {
		newTail = newTail.Next
	}
	newHead := newTail.Next
	newTail.Next = nil
	return newHead
}

// https://leetcode.com/problems/find-good-days-to-rob-the-bank/description/
func goodDaysToRobBank(security []int, time int) []int {
	n := len(security)
	p1, p2 := make([]int, n), make([]int, n)
	for i := 1; i < n; i++ {
		if security[i-1] >= security[i] {
			p1[i] = p1[i-1] + 1
		}
	}
	for i := n - 2; i >= 0; i-- {
		if security[i+1] >= security[i] {
			p2[i] = p2[i+1] + 1
		}
	}
	ans := []int{}
	for i := 0; i < n; i++ {
		if p1[i] >= time && p2[i] >= time {
			ans = append(ans, i)
		}
	}
	return ans
}
