package day255

type Node struct {
	Val      int
	Children []*Node
}

// https://leetcode.com/problems/maximum-depth-of-n-ary-tree/description/
func maxDepth(root *Node) int {
	if root == nil {
		return 0
	}
	q := []*Node{root}
	lvl := 0
	for len(q) > 0 {
		nextQ := []*Node{}

		for _, v := range q {
			for _, u := range v.Children {
				nextQ = append(nextQ, u)
			}
		}

		q = nextQ
		lvl++
	}

	return lvl
}

// https://leetcode.com/problems/minimum-height-trees/description/
func findMinHeightTrees(n int, edges [][]int) []int {
	if n < 2 {
		centroids := []int{}
		for i := 0; i < n; i++ {
			centroids = append(centroids, i)
		}
		return centroids
	}

	adj := make([]map[int]bool, n)
	for i := 0; i < n; i++ {
		adj[i] = map[int]bool{}
	}

	for _, edge := range edges {
		start, end := edge[0], edge[1]
		adj[start][end] = true
		adj[end][start] = true
	}

	leaves := []int{}
	for i := 0; i < n; i++ {
		if len(adj[i]) == 1 {
			leaves = append(leaves, i)
		}
	}

	remainingNode := n

	for remainingNode > 2 {
		remainingNode -= len(leaves)
		newLeaves := []int{}

		for _, leaf := range leaves {
			next := adj[leaf]
			for n, _ := range next {
				delete(adj[n], leaf)
				if len(adj[n]) == 1 {
					newLeaves = append(newLeaves, n)
				}
				break
			}

		}

		leaves = newLeaves
	}

	return leaves
}

// https://leetcode.com/problems/peak-index-in-a-mountain-array/description/
func peakIndexInMountainArray(arr []int) int {
	lo, hi := 0, len(arr)-1
	for lo < hi {
		mid := (lo + hi) / 2
		if arr[mid] < arr[mid+1] {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return lo
}
