package day340

// https://leetcode.com/problems/parallel-courses-iii/description/
func minimumTime(n int, relations [][]int, time []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	adj := map[int][]int{}
	for i := 0; i < n; i++ {
		adj[i] = []int{}
	}
	for _, rel := range relations {
		adj[rel[0]-1] = append(adj[rel[0]-1], rel[1]-1)
	}

	memo := map[int]int{}
	var dfs func(int) int
	dfs = func(node int) int {
		if _, ok := memo[node]; ok {
			return memo[node]
		}

		if len(adj[node]) == 0 {
			return time[node]
		}

		ans := 0
		for _, next := range adj[node] {
			ans = max(ans, dfs(next))
		}

		memo[node] = time[node] + ans
		return memo[node]
	}

	ans := 0
	for node := 0; node < n; node++ {
		ans = max(ans, dfs(node))
	}
	return ans
}
