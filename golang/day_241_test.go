package main

// https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/description/
func distanceK(root *TreeNode, target *TreeNode, k int) []int {
	graph := map[int][]int{}
	var buildGraph func(*TreeNode, *TreeNode)

	buildGraph = func(cur *TreeNode, parent *TreeNode) {
		if cur == nil {
			return
		}

		if parent != nil {
			if graph[parent.Val] == nil {
				graph[parent.Val] = []int{}
			}
			if graph[cur.Val] == nil {
				graph[cur.Val] = []int{}
			}

			graph[parent.Val] = append(graph[parent.Val], cur.Val)
			graph[cur.Val] = append(graph[cur.Val], parent.Val)
		}

		buildGraph(cur.Left, cur)
		buildGraph(cur.Right, cur)
	}

	buildGraph(root, nil)

	curLvl := []int{target.Val}
	visited := map[int]bool{}

	visited[target.Val] = true

	for len(curLvl) > 0 {

		if k == 0 {
			return curLvl
		}

		nextLvl := []int{}

		for _, cur := range curLvl {
			for _, next := range graph[cur] {
				if !visited[next] {
					visited[next] = true
					nextLvl = append(nextLvl, next)
				}
			}
		}

		curLvl = nextLvl
		k--

	}

	return []int{}
}

// https://leetcode.com/problems/find-if-path-exists-in-graph/description/
func validPath(n int, edges [][]int, source int, destination int) bool {
	graph := map[int][]int{}

	for _, e := range edges {
		if graph[e[0]] == nil {
			graph[e[0]] = []int{}
		}
		if graph[e[1]] == nil {
			graph[e[1]] = []int{}
		}

		graph[e[0]] = append(graph[e[0]], e[1])
		graph[e[1]] = append(graph[e[1]], e[0])
	}

	lvl := []int{source}
	seen := map[int]bool{}
	seen[source] = true

	for len(lvl) > 0 {
		nextLvl := []int{}

		for _, cur := range lvl {

			if cur == destination {
				return true
			}

			for _, next := range graph[cur] {
				if !seen[next] {
					seen[next] = true
					nextLvl = append(nextLvl, next)
				}
			}
		}

		lvl = nextLvl

	}

	return false
}
