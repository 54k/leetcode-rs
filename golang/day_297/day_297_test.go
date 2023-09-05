package day297

// https://leetcode.com/problems/copy-list-with-random-pointer/description
type Node struct {
	Val    int
	Next   *Node
	Random *Node
}

func copyRandomListGraph(head *Node) *Node {
	cache := map[*Node]*Node{}
	var copy func(*Node) *Node
	copy = func(root *Node) *Node {
		if root == nil {
			return root
		}

		if _, ok := cache[root]; ok {
			return cache[root]
		}

		node := &Node{root.Val, nil, nil}
		cache[root] = node
		node.Next = copy(root.Next)
		node.Random = copy(root.Random)

		return node
	}

	return copy(head)
}

func copyRandomListLinkedList(head *Node) *Node {
	visited := map[*Node]*Node{}
	if head == nil {
		return head
	}

	getClonedNode := func(n *Node) *Node {
		if n == nil {
			return n
		}
		if _, ok := visited[n]; ok {
			return visited[n]
		}
		visited[n] = &Node{n.Val, nil, nil}
		return visited[n]
	}

	oldNode := head
	newNode := &Node{oldNode.Val, nil, nil}
	visited[oldNode] = newNode

	for oldNode != nil {
		newNode.Random = getClonedNode(oldNode.Random)
		newNode.Next = getClonedNode(oldNode.Next)

		oldNode = oldNode.Next
		newNode = newNode.Next
	}

	return visited[head]
}

func copyRandomListConstantSpace(head *Node) *Node {
	if head == nil {
		return head
	}

	ptr := head
	for ptr != nil {
		newNode := &Node{ptr.Val, nil, nil}
		newNode.Next = ptr.Next
		ptr.Next = newNode
		ptr = newNode.Next
	}

	ptr = head
	for ptr != nil {
		if ptr.Random != nil {
			ptr.Next.Random = ptr.Random.Next
		} else {
			ptr.Next.Random = nil
		}
		ptr = ptr.Next.Next
	}

	ptrOldList := head
	ptrNewList := head.Next
	headOld := head.Next

	for ptrOldList != nil {
		ptrOldList.Next = ptrOldList.Next.Next
		if ptrNewList.Next != nil {
			ptrNewList.Next = ptrNewList.Next.Next
		} else {
			ptrNewList.Next = nil
		}
		ptrOldList = ptrOldList.Next
		ptrNewList = ptrNewList.Next
	}

	return headOld
}
