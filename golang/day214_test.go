package main

func getMinimumDifference(root *TreeNode) int {
	diff := 1000_000_000
	var prev *TreeNode

	abs := func(a, b int) int {
		if a < b {
			return b - a
		}
		return a - b
	}

	var md func(root *TreeNode)
	md = func(root *TreeNode) {
		if root != nil {
			md(root.Left)
			if prev != nil {
				if abs(prev.Val, root.Val) < diff {
					diff = abs(prev.Val, root.Val)
				}
			}
			prev = root
			md(root.Right)
		}
	}

	md(root)
	return diff
}

// https://leetcode.com/problems/sum-of-two-integers/description/
func getSum(a int, b int) int {
	for b != 0 {
		a, b = a^b, (a&b)<<1
	}
	return a
}

// https://leetcode.com/problems/reverse-bits/description/
func reverseBits(num uint32) uint32 {
	var ans uint32
	for i, j := 0, 31; i < j; i, j = i+1, j-1 {
		a, b := (num>>i)&1, (num>>j)&1
		ans |= (a << j)
		ans |= (b << i)
	}
	return ans
}
