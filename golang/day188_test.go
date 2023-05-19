package main

// https://leetcode.com/problems/robot-room-cleaner/description/

import (
	"fmt"
)

// This is the robot's control interface.
// You should not implement it, or speculate about its implementation
type Robot struct {
}

// Returns true if the cell in front is open and robot moves into the cell.
// Returns false if the cell in front is blocked and robot stays in the current cell.
func (robot *Robot) Move() bool {}

// Robot will stay in the same cell after calling TurnLeft/TurnRight.
// Each turn will be 90 degrees.
func (robot *Robot) TurnLeft()  {}
func (robot *Robot) TurnRight() {}

// Clean the current cell.
func (robot *Robot) Clean() {}

func goBack(robo *Robot) {
	robo.TurnRight()
	robo.TurnRight()
	robo.Move()
	robo.TurnRight()
	robo.TurnRight()
}

func backtrack(row, col, d int, directions [][]int, visited map[string]bool, robo *Robot) {
	visited[fmt.Sprintf("%d%d", row, col)] = true
	robo.Clean()

	// going clockwise : 0 'up', 1 'right' 2 'down' 3 'left'
	for i := 0; i < 4; i++ {
		newD := (d + i) % 4
		newRow := row + directions[newD][0]
		newCol := col + directions[newD][1]

		if !visited[fmt.Sprintf("%d%d", newRow, newCol)] && robo.Move() {
			backtrack(newRow, newCol, newD, directions, visited, robo)
			goBack(robo)
		}

		robo.TurnRight()
	}
}

func cleanRoom(robot *Robot) {
	directions := [][]int{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}
	visited := map[string]bool{}
	robo := robot
	backtrack(0, 0, 0, directions, visited, robo)
}

// https://leetcode.com/problems/is-graph-bipartite/description/
func dfs(v int, graph [][]int, colors map[int]int) bool {
	for _, u := range graph[v] {
		if _, ok := colors[u]; !ok {
			colors[u] = 1 - colors[v]
			if !dfs(u, graph, colors) {
				return false
			}
		} else if colors[v] == colors[u] {
			return false
		}
	}
	return true
}

func isBipartite(graph [][]int) bool {
	colors := map[int]int{}
	for v := 0; v < len(graph); v++ {
		if _, ok := colors[v]; !ok {
			colors[v] = 0
			if !dfs(v, graph, colors) {
				return false
			}
		}
	}
	return true
}

// https://leetcode.com/problems/sudoku-solver/description/
func sudokuBacktrack(board [][]byte, row, col int, cache map[string]bool) {
	for board[row][col] != '.' {
		if col < len(board[0]) {
			col++
		} else {
			row++
			col = 0
		}
	}

	if row >= len(board) {
		return
	}

	for num := 0; num < 10; num++ {
		val := byte(rune(num))

		if cache[fmt.Sprintf("r %d v %d", row, val)] ||
			cache[fmt.Sprintf("c %d v %d", col, val)] ||
			cache[fmt.Sprintf("r %d c %d v %d", row/3, col/3, val)] {
			continue
		}

		cache[fmt.Sprintf("r %d v %d", row, val)] = true
		cache[fmt.Sprintf("c %d v %d", col, val)] = true
		cache[fmt.Sprintf("r %d c %d v %d", row/3, col/3, val)] = true
		board[row][col] = val

		sudokuBacktrack(board, row, col, cache)

		cache[fmt.Sprintf("r %d v %d", row, val)] = false
		cache[fmt.Sprintf("c %d v %d", col, val)] = false
		cache[fmt.Sprintf("r %d c %d v %d", row/3, col/3, val)] = false
		board[row][col] = '.'
	}
}

func solveSudoku(board [][]byte) {
	cache := map[string]bool{}
	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board); j++ {
			if board[i][j] != '.' {
				cache[fmt.Sprintf("r %d v %d", i, board[i][j])] = true
				cache[fmt.Sprintf("c %d v %d", j, board[i][j])] = true
				cache[fmt.Sprintf("r %d c %d v %d", i/3, j/3, board[i][j])] = true
			}
		}
	}
	sudokuBacktrack(board, 0, 0, cache)
}

// https://leetcode.com/problems/combinations/description/
func combineRec(n, k, i int, cur *[]int, res *[][]int) {
	if len(*cur) == k {
		dst := make([]int, len(*cur))
		copy(dst, *cur)
		*res = append(*res, dst)
		return
	}

	for j := i; j <= n; j++ {
		*cur = append(*cur, j)
		combineRec(n, k, j+1, cur, res)
		*cur = (*cur)[:len(*cur)-1]
	}
}

func combine(n int, k int) [][]int {
	cur := []int{}
	res := [][]int{}
	combineRec(n, k, 1, &cur, &res)
	return res
}
