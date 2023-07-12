package main

type NodeChildren struct {
	Val      int
	Children []*NodeChildren
}

func levelOrder(root *NodeChildren) [][]int {
	result := [][]int{}
	if root == nil {
		return result
	}

	lvl := []*NodeChildren{root}

	for len(lvl) > 0 {
		result = append(result, []int{})

		next := []*NodeChildren{}

		for _, node := range lvl {
			result[len(result)-1] = append(result[len(result)-1], node.Val)
			for _, ch := range node.Children {
				next = append(next, ch)
			}
		}

		lvl = next
	}

	return result
}
