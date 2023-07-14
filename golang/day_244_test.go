package main

// https://leetcode.com/problems/longest-arithmetic-subsequence-of-given-difference/description/
func longestSubsequence(arr []int, difference int) int {
	dp := map[int]int{}
	answer := 1

	for _, a := range arr {
		beforeA := dp[a-difference]
		dp[a] = beforeA + 1
		if dp[a] > answer {
			answer = dp[a]
		}
	}

	return answer
}

// https://leetcode.com/problems/destroy-sequential-targets/description/
func destroyTargets(nums []int, space int) int {
	m := map[int]int{}
	max := -(1 << 31)
	for _, n := range nums {
		m[n%space]++
		if max < m[n%space] {
			max = m[n%space]
		}
	}

	ans := 1 << 31
	for _, n := range nums {
		if m[n%space] == max && n < ans {
			ans = n
		}
	}

	return ans
}

// https://leetcode.com/problems/sliding-window-maximum/description/
func maxSlidingWindow(nums []int, k int) []int {
	type pair struct {
		i   int
		val int
	}
	ans := []int{}
	q := []pair{}
	for r := 0; r < len(nums); r++ {
		for len(q) > 0 && q[len(q)-1].val <= nums[r] {
			q = q[:len(q)-1]
		}
		q = append(q, pair{r, nums[r]})

		for len(q) > 0 && q[0].i < r-k+1 {
			q = q[1:]
		}

		if r >= k-1 {
			ans = append(ans, q[0].val)
		}
	}

	return ans
}

// https://leetcode.com/problems/network-delay-time/description/
func networkDelayTimeDFS(times [][]int, n int, k int) int {
	type pair struct {
		to   int
		time int
	}

	adj := make([][]pair, n+1)
	for i := 1; i <= n; i++ {
		adj[i] = []pair{}
	}
	for _, edge := range times {
		from, to, time := edge[0], edge[1], edge[2]
		adj[from] = append(adj[from], pair{to, time})
	}

	distances := make([]int, n+1)
	for i := 1; i <= n; i++ {
		distances[i] = 1 << 31
	}

	var dfs func(int, int)
	dfs = func(node, time int) {
		if distances[node] < time {
			return
		}
		distances[node] = time
		for _, next := range adj[node] {
			n, t := next.to, next.time
			if distances[n] > time+t {
				dfs(n, time+t)
			}
		}
	}

	dfs(k, 0)

	ans := -(1 << 31)

	for i := 1; i <= n; i++ {
		if ans < distances[i] {
			ans = distances[i]
		}
	}

	if ans == 1<<31 {
		return -1
	} else {
		return ans
	}
}

func networkDelayTimeBFS(times [][]int, n int, k int) int {
	type pair struct {
		to   int
		time int
	}

	adj := make([][]pair, n+1)
	for i := 1; i <= n; i++ {
		adj[i] = []pair{}
	}
	for _, edge := range times {
		from, to, time := edge[0], edge[1], edge[2]
		adj[from] = append(adj[from], pair{to, time})
	}

	distances := make([]int, n+1)
	for i := 1; i <= n; i++ {
		distances[i] = 1 << 31
	}

	bfs := func(node int) {
		lvl := []int{node}
		distances[node] = 0

		for len(lvl) > 0 {
			nextLvl := []int{}
			for _, curr := range lvl {
				for _, n := range adj[curr] {
					to, time := n.to, n.time
					if distances[to] > time+distances[curr] {
						distances[to] = time + distances[curr]
						nextLvl = append(nextLvl, to)
					}
				}
			}

			lvl = nextLvl
		}
	}

	bfs(k)

	ans := -(1 << 31)

	for i := 1; i <= n; i++ {
		if ans < distances[i] {
			ans = distances[i]
		}
	}

	if ans == 1<<31 {
		return -1
	} else {
		return ans
	}
}
