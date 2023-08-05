package day266

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// https://leetcode.com/problems/unique-binary-search-trees-ii/editorial/
func generateTrees(n int) []*TreeNode {
	type pair struct {
		left  int
		right int
	}
	memo := map[pair][]*TreeNode{}

	var generate func(int, int) []*TreeNode
	generate = func(start, end int) []*TreeNode {
		res := []*TreeNode{}

		if start > end {
			res = append(res, nil)
			return res
		}

		key := pair{start, end}
		if _, ok := memo[key]; ok {
			return memo[key]
		}

		for i := start; i <= end; i++ {
			leftSubTree := generate(start, i-1)
			rightSubTree := generate(i+1, end)

			for _, l := range leftSubTree {
				for _, r := range rightSubTree {
					res = append(res, &TreeNode{i, l, r})
				}
			}
		}

		memo[key] = res
		return res
	}

	return generate(1, n)
}

// https://leetcode.com/problems/greatest-common-divisor-of-strings/description/
func gcdOfStrings(str1 string, str2 string) string {
	panic("todo")
}
