package day424

// https://leetcode.com/problems/amount-of-time-for-binary-tree-to-be-infected/description/
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func amountOfTime(root *TreeNode, start int) int {
	adj := map[int][]int{}
	var dfs func(*TreeNode, *TreeNode)
	dfs = func(n *TreeNode, p *TreeNode) {
		if n == nil {
			return
		}
		if _, ok := adj[n.Val]; !ok {
			adj[n.Val] = []int{}
		}
		if p != nil {
			if _, ok := adj[p.Val]; !ok {
				adj[p.Val] = []int{}
			}
			adj[n.Val] = append(adj[n.Val], p.Val)
			adj[p.Val] = append(adj[p.Val], n.Val)
		}
		dfs(n.Left, n)
		dfs(n.Right, n)
	}

	dfs(root, nil)

	vis := map[int]bool{}
	vis[start] = true

	timer := -1
	cur := []int{start}
	for len(cur) > 0 {
		timer++
		next := []int{}

		for _, v := range cur {
			for _, u := range adj[v] {
				if !vis[u] {
					vis[u] = true
					next = append(next, u)
				}
			}
		}

		cur = next
	}

	return timer
}
