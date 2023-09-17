package day310

import (
	"fmt"
	"testing"
)

// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/
func kWeakestRows(mat [][]int, k int) []int {
	type pair struct {
		s, row int
	}
	m, n := len(mat), len(mat[0])
	getStrength := func(row int) int {
		left, right := 0, n
		for left < right {
			mid := (left + right) / 2
			if mat[row][mid] == 1 {
				left = mid + 1
			} else {
				right = mid
			}
		}
		return left
	}

	rows := make([]pair, m)
	for i := 0; i < len(mat); i++ {
		rows[i] = pair{getStrength(i), i}
	}

	ans := []int{}

	buckets := map[int][]int{}
	for i := 0; i < len(rows); i++ {
		if _, ok := buckets[rows[i].s]; !ok {
			buckets[rows[i].s] = []int{}
		}

		buckets[rows[i].s] = append(buckets[rows[i].s], rows[i].row)
	}

	for strength := 0; k > 0; strength++ {
		if _, ok := buckets[strength]; !ok {
			continue
		}

		for _, v := range buckets[strength] {
			ans = append(ans, v)
			k--
			if k == 0 {
				break
			}
		}
	}

	return ans
}

func TestKWeakestPoints(t *testing.T) {
	res := kWeakestRows([][]int{
		{1, 1, 0, 0, 0},
		{1, 1, 1, 1, 1},
		{1, 0, 0, 0, 0},
		{1, 1, 0, 0, 0},
		{1, 1, 1, 1, 1},
	}, 3)
	fmt.Println(res)

	res = kWeakestRows([][]int{{1, 0, 0, 0}, {1, 1, 1, 1}, {1, 0, 0, 0}, {1, 0, 0, 0}}, 2)
	fmt.Println(res)
}
