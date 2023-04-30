package main

import (
	"fmt"
	"testing"
)

type UF struct {
	representatives []int
	sizes           []int
	components      int
}

func newUF(n int) *UF {
	components := n
	n++
	uf := &UF{
		representatives: make([]int, n),
		sizes:           make([]int, n),
		components:      components,
	}
	for i := 0; i < n; i++ {
		uf.representatives[i] = i
		uf.sizes[i] = 1
	}
	return uf
}

func (uf *UF) find(x int) int {
	if uf.representatives[x] != x {
		uf.representatives[x] = uf.find(uf.representatives[x])
	}
	return uf.representatives[x]
}

func (uf *UF) union(x, y int) int {
	px := uf.find(x)
	py := uf.find(y)

	if px == py {
		return 0
	}
	if uf.sizes[px] < uf.sizes[py] {
		uf.representatives[px] = py
		uf.sizes[py] += uf.sizes[px]
	} else {
		uf.representatives[py] = px
		uf.sizes[px] += uf.sizes[py]
	}
	uf.components--
	return 1
}

func (uf *UF) isConnected() bool {
	return uf.components == 1
}

// https://leetcode.com/problems/remove-max-number-of-edges-to-keep-graph-fully-traversable/editorial/
func maxNumEdgesToRemove(n int, edges [][]int) int {
	alice, bob, edgesRequired := newUF(n), newUF(n), 0
	for _, e := range edges {
		if e[0] == 3 {
			edgesRequired += alice.union(e[1], e[2]) | bob.union(e[1], e[2])
		}
	}
	for _, e := range edges {
		if e[0] == 1 {
			edgesRequired += alice.union(e[1], e[2])
		} else if e[0] == 2 {
			edgesRequired += bob.union(e[1], e[2])
		}
	}
	if alice.isConnected() && bob.isConnected() {
		return len(edges) - edgesRequired
	}
	return -1
}

func TestMaxNumEdgesToRemove(t *testing.T) {
	fmt.Printf("maxNumEdgesToRemove(3, [][]int{{3, 1, 2}, {3, 2, 3}, {1, 1, 3}, {1, 2, 4}, {1, 1, 2}, {2, 3, 4}}): %v\n",
		maxNumEdgesToRemove(4, [][]int{{3, 1, 2}, {3, 2, 3}, {1, 1, 3}, {1, 2, 4}, {1, 1, 2}, {2, 3, 4}}))
}
