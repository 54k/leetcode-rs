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
	return 0
}
