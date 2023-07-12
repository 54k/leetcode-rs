package main

type NodeChildren struct {
	Val      int
	Children []*NodeChildren
}

func levelOrder(root *NodeChildren) [][]int {
	result := [][]int{}
	if root == nil {
		return result
	}

	lvl := []*NodeChildren{root}

	for len(lvl) > 0 {
		result = append(result, []int{})

		next := []*NodeChildren{}

		for _, node := range lvl {
			result[len(result)-1] = append(result[len(result)-1], node.Val)
			for _, ch := range node.Children {
				next = append(next, ch)
			}
		}

		lvl = next
	}

	return result
}

// https://leetcode.com/problems/rotting-oranges/description/
func orangesRotting(grid [][]int) int {
	type pair struct {
		r, c int
	}

	queue := []*pair{}
	freshOranges := 0
	ROWS, COLS := len(grid), len(grid[0])

	for r := 0; r < ROWS; r++ {
		for c := 0; c < COLS; c++ {
			if grid[r][c] == 2 {
				queue = append(queue, &pair{r, c})
			} else if grid[r][c] == 1 {
				freshOranges++
			}
		}
	}

	queue = append(queue, &pair{-1, -1})

	minutesElapsed := -1
	directions := [][]int{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

	for len(queue) > 0 {
		row, col := queue[0].r, queue[0].c
		queue = queue[1:]
		// we finish one round of processing
		if row == -1 {
			minutesElapsed++
			// to avoid the endless loop
			if len(queue) > 0 {
				queue = append(queue, &pair{-1, -1})
			}
		} else {
			for _, d := range directions {
				nRow, nCol := row+d[0], col+d[1]

				if nRow >= 0 && nRow < ROWS && nCol >= 0 && nCol < COLS {
					if grid[nRow][nCol] == 1 {
						// this orange would be contaminated
						grid[nRow][nCol] = 2
						freshOranges--
						queue = append(queue, &pair{nRow, nCol})
					}
				}
			}
		}
	}

	if freshOranges == 0 {
		return minutesElapsed
	} else {
		return -1
	}
}

func orangesRottingInPlace(grid [][]int) int {
	ROWS, COLS := len(grid), len(grid[0])
	timestamp := 2
	directions := [][]int{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

	runRottingProcess := func() bool {
		toBeContinued := false

		for row := 0; row < ROWS; row++ {
			for col := 0; col < COLS; col++ {
				if grid[row][col] == timestamp {
					for _, d := range directions {
						nRow, nCol := row+d[0], col+d[1]
						if nRow >= 0 && nRow < ROWS && nCol >= 0 && nCol < COLS {
							if grid[nRow][nCol] == 1 {
								grid[nRow][nCol] = timestamp + 1
								toBeContinued = true
							}
						}
					}
				}
			}
		}

		return toBeContinued
	}

	for runRottingProcess() {
		timestamp++
	}

	for _, row := range grid {
		for _, cell := range row {
			if cell == 1 {
				return -1
			}
		}
	}

	return timestamp - 2
}
