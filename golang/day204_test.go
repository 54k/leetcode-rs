package main

import (
	"fmt"
	"testing"
)

// https://leetcode.com/problems/number-of-provinces/description/
func findCircleNum1(isConnected [][]int) int {
	count := 0
	visited := make([]bool, len(isConnected))
	for i := 0; i < len(isConnected); i++ {
		if visited[i] {
			continue
		}
		visited[i] = true
		lvl := []int{i}
		for len(lvl) > 0 {
			next := []int{}
			for _, v := range lvl {
				for u, x := range isConnected[v] {
					if !visited[u] && x == 1 {
						visited[u] = true
						next = append(next, u)
					}
				}
			}
			lvl = next
		}
		count++
	}
	return count
}

type uf struct {
	repr []int
	rank []int
	n    int
}

func newUf(n int) *uf {
	repr := make([]int, n)
	rank := make([]int, n)
	for i := 0; i < n; i++ {
		repr[i] = i
		rank[i] = 1
	}
	return &uf{repr, rank, n}
}
func (uf *uf) find(x int) int {
	if uf.repr[x] != x {
		uf.repr[x] = uf.find(uf.repr[x])
	}
	return uf.repr[x]
}
func (uf *uf) union(x, y int) {
	px, py := uf.find(x), uf.find(y)
	if px == py {
		return
	}
	if uf.rank[px] > uf.rank[py] {
		px, py = py, px
	}
	uf.repr[px] = py
	uf.rank[py] += uf.rank[px]
	uf.n--
}

func findCircleNum2(isConnected [][]int) int {
	uf := newUf(len(isConnected))
	for i := 0; i < len(isConnected); i++ {
		for u, v := range isConnected[i] {
			if v == 1 {
				uf.union(i, u)
			}
		}
	}
	return uf.n
}

// https://leetcode.com/problems/minimum-path-sum/description/
func minPathSum(grid [][]int) int {
	min := func(x, y int) int {
		if x < y {
			return x
		}
		return y
	}
	n := len(grid)
	m := len(grid[0])

	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {

			if i == 0 && j != 0 {
				grid[i][j] += grid[i][j-1]
			} else if i != 0 && j == 0 {
				grid[i][j] += grid[i-1][j]
			} else if i != 0 && j != 0 {
				grid[i][j] += min(grid[i-1][j], grid[i][j-1])
			}

		}
	}

	return grid[n-1][m-1]
}

// https://leetcode.com/problems/minimum-falling-path-sum/description/
func minFallingPathSum(matrix [][]int) int {
	min := func(x, y int) int {
		if x < y {
			return x
		}
		return y
	}
	n, m := len(matrix), len(matrix[0])
	for i := 1; i < n; i++ {
		for j := 0; j < m; j++ {
			if j > 0 && j < m-1 {
				matrix[i][j] += min(matrix[i-1][j], min(matrix[i-1][j-1], matrix[i-1][j+1]))
			} else if j > 0 && j == m-1 {
				matrix[i][j] += min(matrix[i-1][j], matrix[i-1][j-1])
			} else if j == 0 && j < m-1 {
				matrix[i][j] += min(matrix[i-1][j], matrix[i-1][j+1])
			} else {
				matrix[i][j] += matrix[i-1][j]
			}
		}
	}
	ans := matrix[n-1][0]
	for i := 1; i < n; i++ {
		ans = min(ans, matrix[n-1][i])
	}
	return ans
}

func TestFindCircleNum2(t *testing.T) {
	ans := findCircleNum2([][]int{
		{1, 1, 1}, {1, 1, 1}, {1, 1, 1},
	})
	fmt.Println(ans)
}
