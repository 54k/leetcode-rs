package day309

// https://leetcode.com/problems/shortest-path-visiting-all-nodes/description
func shortestPathLengthBFS(graph [][]int) int {
	type pair struct {
		node, mask int
	}

	n := len(graph)
	endMask := 1<<n - 1
	visited := [][]bool{}
	for i := 0; i < n; i++ {
		visited = append(visited, make([]bool, endMask+1))
	}

	queue := []pair{}
	for i := 0; i < n; i++ {
		queue = append(queue, pair{i, 1 << i})
		visited[i][1<<i] = true
	}

	steps := 0

	for len(queue) > 0 {
		nextQueue := []pair{}

		for _, p := range queue {
			node, mask := p.node, p.mask
			if mask == endMask {
				return steps
			}

			for _, neighbor := range graph[node] {
				neighborMask := mask | (1 << neighbor)

				if !visited[neighbor][neighborMask] {
					visited[neighbor][neighborMask] = true
					nextQueue = append(nextQueue, pair{neighbor, neighborMask})
				}
			}
		}

		queue = nextQueue
		steps++
	}

	return -1
}

func shortestPathLengthFloydWarshall(graph [][]int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	distMatrix := make([][]int, len(graph))
	for i := 0; i < len(graph); i++ {
		distMatrix[i] = make([]int, len(graph))
		for j := 0; j < len(graph); j++ {
			distMatrix[i][j] = int(10e5)
		}
	}

	for i := 0; i < len(graph); i++ {
		distMatrix[i][i] = 0
		for _, j := range graph[i] {
			distMatrix[i][j] = 1
		}
	}

	for k := 0; k < len(graph); k++ {
		for i := 0; i < len(graph); i++ {
			for j := 0; j < len(graph); j++ {
				if distMatrix[i][j] > distMatrix[i][k]+distMatrix[k][j] {
					distMatrix[i][j] = distMatrix[i][k] + distMatrix[k][j]
				}
			}
		}
	}

	endMask := (1 << len(graph)) - 1
	memo := [][]int{}
	for i := 0; i < len(graph); i++ {
		memo = append(memo, make([]int, endMask+1))
		for j := 0; j < endMask; j++ {
			memo[i][j] = -1
		}
	}

	var dfs func(int, int) int
	dfs = func(node int, mask int) int {
		if mask == endMask {
			return 0
		}

		if memo[node][mask] != -1 {
			return memo[node][mask]
		}

		minDist := int(10e5)
		for next := 0; next < len(graph); next++ {
			if mask&(1<<next) == 0 {
				minDist = min(minDist, dfs(next, mask|(1<<next))+distMatrix[node][next])
			}
		}

		memo[node][mask] = minDist
		return minDist
	}

	ans := int(10e6)
	for start := 0; start < len(graph); start++ {
		ans = min(ans, dfs(start, 1<<start))
	}

	return ans
}
