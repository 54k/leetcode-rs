package day338

// https://leetcode.com/problems/pascals-triangle/description/
func generate(numRows int) [][]int {
	triangle := [][]int{}
	triangle = append(triangle, []int{1})
	for row := 1; row < numRows; row++ {
		prev := triangle[len(triangle)-1]
		next := []int{1}
		for i := 1; i < row; i++ {
			next = append(next, prev[i-1]+prev[i])
		}
		next = append(next, 1)
		triangle = append(triangle, next)
	}
	return triangle
}

// https://leetcode.com/problems/next-greater-node-in-linked-list/description/
type ListNode struct {
	Val  int
	Next *ListNode
}

func nextLargerNodes(head *ListNode) []int {
	type pair struct {
		idx, val int
	}
	ans := []int{}
	stack := []pair{}
	for head != nil {
		for len(stack) > 0 && stack[len(stack)-1].val < head.Val {
			top := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			ans[top.idx] = head.Val
		}
		ans = append(ans, 0)
		stack = append(stack, pair{len(ans) - 1, head.Val})
		head = head.Next
	}
	return ans
}
