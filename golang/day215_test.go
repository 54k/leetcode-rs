package main

// https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/description/
func maxLevelSum(root *TreeNode) int {
	lvls := map[int]int{}
	lvl := []*TreeNode{root}
	clvl := 1
	for len(lvl) > 0 {
		next := []*TreeNode{}
		for _, v := range lvl {
			lvls[clvl] += v.Val
			if v.Left != nil {
				next = append(next, v.Left)
			}
			if v.Right != nil {
				next = append(next, v.Right)
			}
		}
		lvl = next
		clvl++
	}

	// fmt.Println(lvls)

	ansk, ansv := 0, -1000000000
	for l := 1; l < clvl; l++ {
		if lvls[l] > ansv {
			ansv = lvls[l]
			ansk = l
		}
	}
	return ansk
}
