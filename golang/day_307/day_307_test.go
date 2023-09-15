package day307

import (
	"sort"
)

// https://leetcode.com/problems/min-cost-to-connect-all-points
func minCostConnectPointsKruskal(points [][]int) int {
	dsu := make([]int, len(points))
	rank := make([]int, len(points))

	for i := 0; i < len(points); i++ {
		dsu[i] = i
		rank[i] = 1
	}

	var find func(x int) int
	find = func(x int) int {
		if dsu[x] != x {
			dsu[x] = find(dsu[x])
		}
		return dsu[x]
	}

	union := func(x, y int) {
		x = find(x)
		y = find(y)
		if x == y {
			return
		}
		if rank[x] > rank[y] {
			x, y = y, x
		}
		dsu[x] = y
		rank[y] += rank[x]
	}

	abs := func(i int) int {
		if i < 0 {
			return -i
		}
		return i
	}

	type Edge struct {
		from int
		to   int
		dist int
	}

	edges := make([]Edge, len(points))
	for i := 0; i < len(points); i++ {
		for j := i + 1; j < len(points); j++ {
			x1, y1 := points[i][0], points[i][1]
			x2, y2 := points[j][0], points[j][1]
			dist := abs(x1-x2) + abs(y1-y2)
			edge := Edge{i, j, dist}
			edges = append(edges, edge)
		}
	}

	sort.Slice(edges, func(x, y int) bool {
		return edges[x].dist < edges[y].dist
	})

	total := 0
	for _, edge := range edges {
		if find(edge.from) != find(edge.to) {
			union(edge.from, edge.to)
			total += edge.dist
		}
	}

	return total
}

func minCostConnectPointsPrimOptimized(points [][]int) int {
	abs := func(a int) int {
		if a < 0 {
			return -a
		}
		return a
	}

	n := len(points)
	cost, used := 0, 0

	inMST := make([]bool, n)
	dist := make([]int, n)
	dist[0] = 0
	for i := 1; i < n; i++ {
		dist[i] = 1 << 31
	}

	for used < n {
		minCost := 1 << 31
		node := 0
		for i := 0; i < n; i++ {
			if !inMST[i] && minCost > dist[i] {
				minCost = dist[i]
				node = i
			}
		}

		cost += minCost
		used++
		inMST[node] = true

		for next := 0; next < n; next++ {
			weight := abs(points[node][0]-points[next][0]) + abs(points[node][1]-points[next][1])
			if !inMST[next] && dist[next] > weight {
				dist[next] = weight
			}
		}
	}
	return cost
}
