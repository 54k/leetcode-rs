package main

// https://leetcode.com/problems/shortest-path-in-binary-matrix/
func shortestPathBinaryMatrix(grid [][]int) int {
	if grid[0][0] == 1 {
		return -1
	}

	type pair struct {
		x, y int
	}
	lvl := []pair{{0, 0}}
	dist := 1

	for len(lvl) > 0 {
		next := []pair{}

		for _, val := range lvl {
			x, y := val.x, val.y
			if x == len(grid)-1 && y == len(grid[0])-1 {
				return dist
			}

			for i := -1; i < 2; i++ {
				for j := -1; j < 2; j++ {
					if i == 0 && j == 0 {
						continue
					}

					nx, ny := x+i, y+j

					if 0 <= nx && nx < len(grid) && 0 <= ny && ny < len(grid[0]) && grid[nx][ny] == 0 {
						grid[nx][ny] = 1
						next = append(next, pair{nx, ny})
					}
				}
			}
		}

		lvl = next
		dist++
	}

	return -1
}
