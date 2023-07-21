package day251

import "fmt"

// https://leetcode.com/problems/number-of-longest-increasing-subsequence/description/
func findNumberOfLIS(nums []int) int {
	length := make([]int, len(nums))
	count := make([]int, len(nums))

	for i := 0; i < len(nums); i++ {
		length[i] = 1
		count[i] = 1
	}

	for i := 0; i < len(nums); i++ {
		for j := 0; j < i; j++ {
			if nums[j] < nums[i] {
				if length[j]+1 > length[i] {
					length[i] = length[j] + 1
					count[i] = 0
				}
				if length[j]+1 == length[i] {
					count[i] += count[j]
				}
			}
		}
	}

	maxLength := 0
	result := 0

	for _, l := range length {
		if l > maxLength {
			maxLength = l
		}
	}

	for i := 0; i < len(nums); i++ {
		if length[i] == maxLength {
			result += count[i]
		}
	}

	return result
}

// https://leetcode.com/problems/cheapest-flights-within-k-stops/description/
func findCheapestPrice(n int, flights [][]int, src int, dst int, k int) int {
	type pair struct {
		to     int
		weight int
	}
	adj := map[int][]pair{}
	for _, f := range flights {
		if _, ok := adj[f[0]]; !ok {
			adj[f[0]] = []pair{}
		}
		adj[f[0]] = append(adj[f[0]], pair{f[1], f[2]})
	}
	dist := make([]int, n)
	for i := 0; i < len(dist); i++ {
		dist[i] = 1 << 30
	}

	lvl := []pair{pair{src, 0}}
	stops := 0

	for stops <= k && len(lvl) > 0 {
		newLvl := []pair{}

		for _, el := range lvl {
			node, d := el.to, el.weight
			if _, ok := adj[node]; !ok {
				continue
			}

			for _, next := range adj[node] {
				nnode, nprice := next.to, next.weight
				if nprice+d >= dist[nnode] {
					continue
				}
				dist[nnode] = nprice + d
				newLvl = append(newLvl, pair{nnode, dist[nnode]})
			}
		}

		lvl = newLvl
		stops++
	}

	if dist[dst] == 1<<30 {
		return -1
	}
	return dist[dst]
}

func findCheapestPriceBellmanFord(n int, flights [][]int, src int, dst int, k int) int {
	dist := make([]int, n)
	for i := 0; i < n; i++ {
		dist[i] = 1 << 30
	}
	dist[src] = 0

	for i := 0; i <= k; i++ {
		temp := make([]int, n)
		copy(temp, dist)
		fmt.Println(temp)
		for _, flight := range flights {
			if dist[flight[0]] != 1<<30 {
				if temp[flight[1]] > dist[flight[0]]+flight[2] {
					temp[flight[1]] = dist[flight[0]] + flight[2]
				}
			}
		}

		dist = temp
	}

	if dist[dst] == 1<<30 {
		return -1
	}
	return dist[dst]
}
