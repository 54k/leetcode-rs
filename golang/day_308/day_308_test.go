package day308

import (
	"fmt"
	"testing"
)

// https://leetcode.com/problems/path-with-minimum-effort/description
func minimumEffortPathBfs(heights [][]int) int {
	type pair struct {
		x, y int
	}

	abs := func(a int) int {
		if a < 0 {
			return -a
		}
		return a
	}

	bfs := func(effort int) bool {
		visited := [][]bool{}
		for i := 0; i < len(heights); i++ {
			visited = append(visited, make([]bool, len(heights[i])))
		}

		nextMoves := func(p pair) []pair {
			moves := []pair{{-1, 0}, {1, 0}, {0, 1}, {0, -1}}
			nextMoves := []pair{}
			for _, dir := range moves {
				nx, ny := p.x+dir.x, p.y+dir.y
				if nx < 0 || nx >= len(heights) || ny < 0 || ny >= len(heights[0]) {
					continue
				}
				if visited[nx][ny] {
					continue
				}
				diff := abs(heights[p.x][p.y] - heights[nx][ny])
				if diff > effort {
					continue
				}
				nextMoves = append(nextMoves, pair{nx, ny})
			}
			return nextMoves
		}

		curr := []pair{{0, 0}}
		visited[0][0] = true

		for len(curr) > 0 {
			next := []pair{}
			for _, p := range curr {
				if p.x == len(heights)-1 && p.y == len(heights[0])-1 {
					return true
				}
				for _, m := range nextMoves(p) {
					visited[m.x][m.y] = true
					next = append(next, m)
				}
			}
			curr = next
		}

		return false
	}

	left, right := 0, 1<<31
	minEffort := 0
	for left <= right {
		mid := left + (right-left)/2
		if bfs(mid) {
			right = mid - 1
			minEffort = mid
		} else {
			left = mid + 1
		}
	}

	return minEffort
}

func minimumEffortPathDfs(heights [][]int) int {
	type pair struct {
		x, y int
	}

	abs := func(a int) int {
		if a < 0 {
			return -a
		}
		return a
	}

	canReach := func(effort int) bool {
		visited := [][]bool{}
		for i := 0; i < len(heights); i++ {
			visited = append(visited, make([]bool, len(heights[0])))
		}
		visited[0][0] = true

		validMoves := func(cur pair) []pair {
			dirs := []pair{{-1, 0}, {1, 0}, {0, 1}, {0, -1}}
			moves := []pair{}
			for _, dir := range dirs {
				nx, ny := cur.x+dir.x, cur.y+dir.y
				if nx < 0 || nx >= len(heights) || ny < 0 || ny >= len(heights[0]) {
					continue
				}
				if visited[nx][ny] {
					continue
				}
				diff := abs(heights[cur.x][cur.y] - heights[nx][ny])
				if diff <= effort {
					moves = append(moves, pair{nx, ny})
				}
			}
			return moves
		}

		var dfs func(pair) bool
		dfs = func(v pair) bool {
			if v.x == len(heights)-1 && v.y == len(heights[0])-1 {
				return true
			}

			for _, u := range validMoves(v) {
				visited[u.x][u.y] = true
				if dfs(u) {
					return true
				}
			}
			return false
		}

		return dfs(pair{0, 0})
	}

	left, right := 0, 1<<31
	for left < right {
		mid := left + (right-left)/2
		if canReach(mid) {
			right = mid
		} else {
			left = mid + 1
		}
	}

	return left
}

func TestMinEffortPath(t *testing.T) {
	res := minimumEffortPathBfs([][]int{{1, 2, 2}, {3, 8, 2}, {5, 3, 5}})
	fmt.Println(res) // 2
}

// https://leetcode.com/problems/path-with-maximum-minimum-value/description/
func maximumMinimumPathBruteForce(grid [][]int) int {
	row, col := len(grid), len(grid[0])

	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	hasPathWithThreshold := func(threshold int) bool {
		isValidMove := func(x, y int) bool {
			return x >= 0 && y >= 0 && x < row && y < col
		}

		type pair struct {
			x, y int
		}
		bfs := func(threshold int) bool {
			visited := [][]bool{}
			for i := 0; i < row; i++ {
				visited = append(visited, make([]bool, col))
			}
			visited[0][0] = true

			cur := []pair{{0, 0}}
			for len(cur) > 0 {
				next := []pair{}

				for _, c := range cur {
					x, y := c.x, c.y

					if x == row-1 && y == col-1 {
						return true
					}

					for _, d := range []pair{{-1, 0}, {1, 0}, {0, 1}, {0, -1}} {
						nx, ny := x+d.x, y+d.y

						if isValidMove(nx, ny) && !visited[nx][ny] && grid[nx][ny] >= threshold {
							visited[nx][ny] = true
							next = append(next, pair{nx, ny})
						}
					}
				}
				cur = next
			}
			return false
		}
		return bfs(threshold)
	}

	// brute force approach
	// threshold := min(grid[0][0], grid[row-1][col-1])
	// for threshold >= 0 {
	// 	if hasPathWithThreshold(threshold) {
	// 		return threshold
	// 	}
	// 	threshold--
	// }

	// return -1

	left := 0
	right := min(grid[1][0], grid[row-1][col-1])

	for left < right {
		mid := (left + right + 1) / 2
		if hasPathWithThreshold(mid) {
			left = mid
		} else {
			right = mid - 1
		}
	}

	return left
}
