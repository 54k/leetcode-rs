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

// https://leetcode.com/problems/sparse-matrix-multiplication/description/
func multiply(mat1 [][]int, mat2 [][]int) [][]int {
	n := len(mat1)
	k := len(mat1[0])
	m := len(mat2[0])

	ans := make([][]int, n)
	for i := 0; i < n; i++ {
		ans[i] = make([]int, m)
	}

	for rowIndex := 0; rowIndex < n; rowIndex++ {
		for elementIndex := 0; elementIndex < k; elementIndex++ {
			if mat1[rowIndex][elementIndex] != 0 {
				for colIndex := 0; colIndex < m; colIndex++ {
					ans[rowIndex][colIndex] += mat1[rowIndex][elementIndex] * mat2[elementIndex][colIndex]
				}
			}
		}
	}

	return ans
}

func multiplyCompressed(mat1 [][]int, mat2 [][]int) [][]int {
	type pair struct {
		val int
		col int
	}

	compress := func(mat [][]int) [][]pair {
		compressed := [][]pair{}
		for r := 0; r < len(mat); r++ {
			row := []pair{}
			for c := 0; c < len(mat[0]); c++ {
				if mat[r][c] == 0 {
					continue
				}
				row = append(row, pair{mat[r][c], c})
			}
			compressed = append(compressed, row)
		}
		return compressed
	}

	A := compress(mat1)
	B := compress(mat2)

	C := make([][]int, len(mat1))
	for i := 0; i < len(mat1); i++ {
		C[i] = make([]int, len(mat2[0]))
	}

	for r := 0; r < len(mat1); r++ {
		for _, p1 := range A[r] {
			v1, col1 := p1.val, p1.col

			for _, p2 := range B[col1] {
				v2, col2 := p2.val, p2.col
				C[r][col2] += v1 * v2
			}
		}
	}

	return C
}
