package main

import (
	"fmt"
	"testing"

	"github.com/emirpasic/gods/sets/treeset"
)

func getSkylineBruteForce(buildings [][]int) [][]int {
	set := treeset.NewWithIntComparator()
	for _, building := range buildings {
		set.Add(building[0])
		set.Add(building[1])
	}
	edges := set.Values()
	edgesToIdx := map[int]int{}
	for i, edge := range edges {
		edgesToIdx[edge.(int)] = i
	}

	heigths := make([]int, len(edges))

	for _, b := range buildings {
		left, right, height := b[0], b[1], b[2]
		leftIdx, rightIdx := edgesToIdx[left], edgesToIdx[right]

		for idx := leftIdx; idx < rightIdx; idx++ {
			if height > heigths[idx] {
				heigths[idx] = height
			}
		}
	}

	ans := [][]int{}
	for i, h := range heigths {
		currHeight, currPos := h, edges[i]

		if len(ans) == 0 || ans[len(ans)-1][1] != currHeight {
			ans = append(ans, []int{currPos.(int), currHeight})
		}
	}
	return ans
}

func getSkylineBruteSweepLine(buildings [][]int) [][]int {
	set := treeset.NewWithIntComparator()
	for _, building := range buildings {
		set.Add(building[0])
		set.Add(building[1])
	}

	ans := [][]int{}
	positions := set.Values()

	for _, p := range positions {
		position := p.(int)
		max_height := 0
		for _, building := range buildings {
			left, right, heigth := building[0], building[1], building[2]
			if left <= position && position < right && max_height < heigth {
				max_height = heigth
			}
		}
		if len(ans) == 0 || ans[len(ans)-1][1] != max_height {
			ans = append(ans, []int{position, max_height})
		}
	}

	return ans
}

// https://leetcode.com/problems/shortest-bridge/
func shortestBridge(grid [][]int) int {
	n := len(grid)
	dirs := [][]int{{-1, 0}, {1, 0}, {0, 1}, {0, -1}}
	queue := [][]int{}
	queue2 := [][]int{}

outer:
	for i, row := range grid {
		for j, col := range row {
			if col == 1 {
				grid[i][j] = 2
				queue = append(queue, []int{i, j})
				break outer
			}
		}
	}

	for len(queue) > 0 {
		new_queue := [][]int{}
		for _, coord := range queue {
			queue2 = append(queue2, coord)
			for _, d := range dirs {
				new_x, new_y := d[0]+coord[0], d[1]+coord[1]
				if 0 <= new_x && new_x < n && 0 <= new_y && new_y < n && grid[new_x][new_y] == 1 {
					new_queue = append(new_queue, []int{new_x, new_y})
					grid[new_x][new_y] = 2
				}
			}
		}
		queue = new_queue
	}

	dist := 0
	for len(queue2) > 0 {
		new_queue := [][]int{}
		for _, coord := range queue2 {
			for _, d := range dirs {
				new_x, new_y := d[0]+coord[0], d[1]+coord[1]
				if 0 <= new_x && new_x < n && 0 <= new_y && new_y < n {
					if grid[new_x][new_y] == 0 {
						new_queue = append(new_queue, []int{new_x, new_y})
						grid[new_x][new_y] = -1
					} else if grid[new_x][new_y] == 1 {
						return dist
					}
				}
			}
		}
		queue2 = new_queue
		dist++
	}

	return dist
}

func TestGetSkyline(t *testing.T) {
	ans := getSkylineBruteForce([][]int{{2, 9, 10}, {3, 7, 15}, {5, 12, 12}, {15, 20, 10}, {19, 24, 8}})
	fmt.Println(ans) // [[2 10] [3 15] [7 12] [12 0] [15 10] [20 8] [24 0]]
}

func TestShortestBridge(t *testing.T) {
	fmt.Println(shortestBridge([][]int{{0, 1, 0}, {0, 0, 0}, {0, 0, 1}})) // 2
}
