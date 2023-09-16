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

func TestMinEffortPath(t *testing.T) {
	res := minimumEffortPathBfs([][]int{{1, 2, 2}, {3, 8, 2}, {5, 3, 5}})
	fmt.Println(res) // 2
}
