package day255

import (
	"fmt"
	"testing"
)

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

// https://leetcode.com/problems/parallel-courses/description/
func minimumSemesters(n int, relations [][]int) int {
	adj := make([][]int, n+1)

	for i := 0; i <= n; i++ {
		adj[i] = []int{}
	}

	for _, r := range relations {
		adj[r[0]] = append(adj[r[0]], r[1])
	}

	maxLength := 1
	vis := make([]int, n+1)

	var dfs func(int) int
	dfs = func(node int) int {
		if vis[node] != 0 {
			return vis[node]
		} else {
			vis[node] = -1
		}

		maxLength := 1
		for _, endNode := range adj[node] {
			len := dfs(endNode)
			if len == -1 {
				return -1
			}
			if len+1 > maxLength {
				maxLength = len + 1
			}
		}
		vis[node] = maxLength
		return maxLength
	}

	for node := 1; node <= n; node++ {
		len := dfs(node)
		if len == -1 {
			return -1
		}
		if len > maxLength {
			maxLength = len
		}
	}

	return maxLength
}

// https://leetcode.com/problems/parallel-courses-ii/description/
func minNumberOfSemesters(n int, relations [][]int, k int) int {
	adj := make([][]int, n)

	for i := 0; i < len(adj); i++ {
		adj[i] = []int{}
	}
	for _, r := range relations {
		f, t := r[0]-1, r[1]-1
		adj[f] = append(adj[f], t)
	}

	memo := make([]int, 1<<n)
	for i := 0; i < len(memo); i++ {
		memo[i] = -1
	}

	popCount := func(n int) int {
		ans := 0
		for n != 0 {
			if n&1 == 1 {
				ans++
			}
			n >>= 1
		}
		return ans
	}

	var dp func(int) int
	dp = func(mask int) int {
		if mask == (1<<n)-1 {
			return 0
		}
		if memo[mask] != -1 {
			return memo[mask]
		}

		degree := make([]int, n)

		for i := 0; i < n; i++ {
			if mask&(1<<i) == 0 {
				for _, nxt := range adj[i] {
					degree[nxt]++
				}
			}
		}

		cm := 0 // candidates mask
		for i := 0; i < n; i++ {
			if degree[i] == 0 && mask&(1<<i) == 0 {
				cm |= 1 << i
			}
		}
		cmCnt := popCount(cm) // num of candidates

		ans := n + 1
		if cmCnt <= k {
			sem := 1 + dp(mask|cm)
			if ans > sem {
				ans = sem
			}
		} else {
			cur := cm // current count of candidates
			for ; cur > 0; cur = (cur - 1) & cm {
				curCnt := popCount(cur)
				if curCnt == k {
					sem := 1 + dp(mask|cur)
					if ans > sem {
						ans = sem
					}
				}
			}
		}

		memo[mask] = ans
		return ans
	}

	return dp(0)
}

func TestMinNumOfSemesters(t *testing.T) {
	ans := minNumberOfSemesters(4, [][]int{{2, 1}, {3, 1}, {1, 4}}, 2)
	fmt.Println(ans)
}
