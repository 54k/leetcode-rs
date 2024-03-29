package day279

// https://leetcode.com/problems/maximal-network-rank/
func maximalNetworkRank(n int, roads [][]int) int {
	degree := make([]int, n)
	mat := make([][]bool, n)
	for i := 0; i < n; i++ {
		mat[i] = make([]bool, n)
	}
	for _, road := range roads {
		mat[road[0]][road[1]] = true
		mat[road[1]][road[0]] = true
		degree[road[0]]++
		degree[road[1]]++
	}

	ans := 0
	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			common := 0
			if mat[i][j] == true {
				common--
			}
			if degree[i]+degree[j]+common > ans {
				ans = degree[i] + degree[j] + common
			}
		}
	}
	return ans
}

// https://leetcode.com/problems/increasing-order-search-tree/description/
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func increasingBST(root *TreeNode) *TreeNode {
	dummy := &TreeNode{0, nil, nil}
	next := dummy
	stack := []*TreeNode{}
	p := root

	for p != nil || len(stack) > 0 {
		for p != nil {
			stack = append(stack, p)
			p = p.Left
		}

		top := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		top.Left = nil
		next.Right = top
		next = top
		p = top.Right
	}
	return dummy.Right
}

// https://leetcode.com/problems/gas-station/description/
func canCompleteCircuit(gas []int, cost []int) int {
	mn := 1 << 60
	sum := 0
	ans := 0
	for i := 0; i < len(gas); i++ {
		if sum < mn {
			ans = i
			mn = sum
		}
		sum += gas[i] - cost[i]
	}
	if sum < 0 {
		return -1
	}
	return ans
}

// https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/description/
func bstFromPreorder(preorder []int) *TreeNode {
	lo := -(1 << 60)
	hi := 1 << 60
	idx := 0
	var helper func(int, int) *TreeNode
	helper = func(lo int, hi int) *TreeNode {
		if idx == len(preorder) {
			return nil
		}

		val := preorder[idx]
		if val <= lo || val >= hi {
			return nil
		}

		idx++

		root := &TreeNode{val, nil, nil}
		root.Left = helper(lo, val)
		root.Right = helper(val, hi)
		return root
	}
	return helper(lo, hi)
}

func bstFromPreorderIter(preorder []int) *TreeNode {
	n := len(preorder)
	if n == 0 {
		return nil
	}
	root := &TreeNode{preorder[0], nil, nil}
	deque := []*TreeNode{}
	deque = append(deque, root)

	for i := 1; i < n; i++ {
		node := deque[len(deque)-1]
		child := &TreeNode{preorder[i], nil, nil}

		for len(deque) > 0 && deque[len(deque)-1].Val < child.Val {
			pop := deque[len(deque)-1]
			deque = deque[:len(deque)-1]
			node = pop
		}
		if node.Val < child.Val {
			node.Right = child
		} else {
			node.Left = child
		}
		deque = append(deque, child)
	}
	return root
}

// https://leetcode.com/problems/buildings-with-an-ocean-view/description/
func findBuildings(heights []int) []int {
	ans := []int{}
	max := 0
	for i := len(heights) - 1; i >= 0; i-- {
		if heights[i] > max {
			max = heights[i]
			ans = append(ans, i)
		}
	}
	for i := 0; i < len(ans)/2; i++ {
		ans[i], ans[len(ans)-i-1] = ans[len(ans)-i-1], ans[i]
	}
	return ans
}

// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/description/
func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	rVal, pVal, qVal := root.Val, p.Val, q.Val
	if pVal < rVal && qVal < rVal {
		return lowestCommonAncestor(root.Left, p, q)
	} else if pVal > rVal && qVal > rVal {
		return lowestCommonAncestor(root.Right, p, q)
	}
	return root
}

func lowestCommonAncestorIter(root, p, q *TreeNode) *TreeNode {
	pVal, qVal := p.Val, q.Val
	node := root
	for node != nil {
		parentVal := node.Val

		if pVal > parentVal && qVal > parentVal {
			node = node.Right
		} else if pVal < parentVal && qVal < parentVal {
			node = node.Left
		} else {
			return node
		}
	}
	return root
}

// https://leetcode.com/problems/verify-preorder-sequence-in-binary-search-tree/description/
func verifyPreorder(preorder []int) bool {
	stack := []int{}
	minLimit := -(1 << 30)
	for i := 0; i < len(preorder); i++ {
		child := preorder[i]
		for len(stack) > 0 && stack[len(stack)-1] < child {
			minLimit = stack[len(stack)-1]
			stack = stack[:len(stack)-1]
		}
		if child <= minLimit {
			return false
		}
		stack = append(stack, child)
	}
	return true
}
