package day307

import "sort"

// https://leetcode.com/problems/min-cost-to-connect-all-points
func minCostConnectPoints(points [][]int) int {
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
