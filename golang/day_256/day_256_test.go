package day256

import "math"

// https://leetcode.com/problems/minimum-speed-to-arrive-on-time/description/
func minSpeedOnTime(dist []int, hour float64) int {
	isValid := func(speed float64) float64 {
		time := 0.0

		for i := 0; i < len(dist); i++ {
			t := float64(dist[i]) / speed
			if i < len(dist)-1 {
				time += math.Ceil(t)
			} else {
				time += t
			}
		}
		return time
	}

	left, right, minSpeed := 1, 10000000, -1

	for left <= right {
		mid := (left + right) / 2
		if isValid(float64(mid)) <= hour {
			minSpeed = mid
			right = mid - 1
		} else {
			left = mid + 1
		}
	}

	return minSpeed
}

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// https://leetcode.com/problems/binary-tree-preorder-traversal/description/
func preorderTraversal(root *TreeNode) []int {
	stack := []*TreeNode{root}
	answer := []int{}

	for len(stack) > 0 {
		curr := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if curr != nil {
			answer = append(answer, curr.Val)
			stack = append(stack, curr.Right)
			stack = append(stack, curr.Left)
		}
	}

	return answer
}
