package day316

import "math"

// https://leetcode.com/problems/champagne-tower/
func champagneTower(poured int, query_row int, query_glass int) float64 {
	a := make([][]float64, 102)
	for i := 0; i < 102; i++ {
		a[i] = make([]float64, 102)
	}

	a[0][0] = float64(poured)
	for r := 0; r <= query_row; r++ {
		for c := 0; c <= r; c++ {
			p := (a[r][c] - 1.0) / 2.0
			if p > 0.0 {
				a[r+1][c] += p
				a[r+1][c+1] += p
			}
		}
	}

	return math.Min(a[query_row][query_glass], 1.0)
}
