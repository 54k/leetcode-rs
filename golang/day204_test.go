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

func TestFindCircleNum2(t *testing.T) {
	ans := findCircleNum2([][]int{
		{1, 1, 1}, {1, 1, 1}, {1, 1, 1},
	})
	fmt.Println(ans)
}
