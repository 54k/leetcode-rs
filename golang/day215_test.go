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

// https://leetcode.com/problems/gray-code/description/
func grayCode(n int) []int {
	result := []int{0}
	visited := map[int]bool{0: true}
	var dfs func(v int) bool
	dfs = func(v int) bool {
		if len(result) == 1<<n {
			return true
		}
		current := result[len(result)-1]
		for i := 0; i < n; i++ {
			next := current ^ (1 << i)
			if !visited[next] {
				visited[next] = true
				result = append(result, next)

				if dfs(n) {
					return true
				}

				visited[next] = false
				result = result[:len(result)-1]
			}
		}
		return false
	}
	dfs(n)
	return result
}
