package main

import (
	"fmt"
	"testing"
)

var (
	cols  = make(map[int]bool)
	diag1 = make(map[int]bool)
	diag2 = make(map[int]bool)
	size  int
)

func totalNQueensRec(n int) int {
	if n == size {
		return 1
	}

	row := n
	ans := 0

	for col := 0; col < size; col++ {
		d1 := row + col
		d2 := row - col
		fmt.Println(d1, d2, col, row)

		if diag1[d1] || diag2[d2] || cols[col] {
			continue
		}

		diag1[d1] = true
		diag2[d2] = true
		cols[col] = true

		ans += totalNQueensRec(row + 1)

		diag1[d1] = false
		diag2[d2] = false
		cols[col] = false

	}

	return ans
}

func totalNQueens(n int) int {
	size = n
	return totalNQueensRec(0)
}

func TestNQueens(t *testing.T) {
	fmt.Println(totalNQueens(4))
}
