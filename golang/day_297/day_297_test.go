package day297

// https://leetcode.com/problems/copy-list-with-random-pointer/description
type Node struct {
	Val    int
	Next   *Node
	Random *Node
}

func copyRandomList(head *Node) *Node {
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
