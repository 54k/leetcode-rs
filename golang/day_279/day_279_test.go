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
