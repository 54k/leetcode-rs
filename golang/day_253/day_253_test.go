package day253

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// https://leetcode.com/problems/all-possible-full-binary-trees/description/
func allPossibleFBT(n int) []*TreeNode {
	memo := map[int][]*TreeNode{}

	var dfs func(int) []*TreeNode
	dfs = func(n int) []*TreeNode {
		if n%2 == 0 {
			return []*TreeNode{}
		}

		if n == 1 {
			return []*TreeNode{&TreeNode{0, nil, nil}}
		}

		if _, ok := memo[n]; ok {
			return memo[n]
		}

		res := []*TreeNode{}

		for i := 1; i < n; i += 2 {
			left := dfs(i)
			right := dfs(n - i - 1)

			for _, l := range left {
				for _, r := range right {
					root := &TreeNode{0, l, r}
					res = append(res, root)
				}
			}
		}

		memo[n] = res
		return memo[n]
	}

	return dfs(n)
}

func allPossibleFBTIterative(n int) []*TreeNode {
	if n%2 == 0 {
		return []*TreeNode{}
	}

	dp := make([][]*TreeNode, n+1)

	for i := 0; i < len(dp); i++ {
		dp[i] = []*TreeNode{}
	}

	dp[1] = []*TreeNode{&TreeNode{0, nil, nil}}

	for count := 3; count <= n; count += 2 {
		for i := 1; i < count-1; i += 2 {
			j := count - 1 - i

			for _, left := range dp[i] {
				for _, right := range dp[j] {
					root := &TreeNode{0, left, right}
					dp[count] = append(dp[count], root)
				}
			}
		}
	}

	return dp[n]
}
