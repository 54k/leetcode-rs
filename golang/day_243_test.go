package main

// https://leetcode.com/problems/course-schedule/description/
func canFinishKanhTopologicalSort(numCourses int, prerequisites [][]int) bool {
	in_degree := make([]int, numCourses)
	adj := make([][]int, numCourses)

	for i := 0; i < numCourses; i++ {
		adj[i] = []int{}
	}

	for _, p := range prerequisites {
		from, to := p[0], p[1]
		in_degree[to]++
		adj[from] = append(adj[from], to)
	}

	q := []int{}
	for i, d := range in_degree {
		if d == 0 {
			q = append(q, i)
		}
	}

	visited := 0
	for len(q) > 0 {
		v := q[0]
		q = q[1:]
		visited++
		for _, u := range adj[v] {
			in_degree[u]--
			if in_degree[u] == 0 {
				q = append(q, u)
			}
		}
	}

	return visited == numCourses
}

func canFinishDFSTopologicalSort(numCourses int, prerequisites [][]int) bool {
	result := []int{}

	seen := make([]int, numCourses)
	adj := make([][]int, numCourses)

	for i := 0; i < numCourses; i++ {
		adj[i] = []int{}
	}

	for _, p := range prerequisites {
		from, to := p[0], p[1]
		adj[from] = append(adj[from], to)
	}

	var dfs func(int) bool
	dfs = func(v int) bool {
		if seen[v] == 1 {
			return false
		}
		seen[v] = 1
		for _, u := range adj[v] {
			if seen[u] != 2 && !dfs(u) {
				return false
			}
		}
		seen[v] = 2
		result = append(result, v)
		return true
	}

	for i := 0; i < numCourses; i++ {
		if seen[i] == 0 && !dfs(i) {
			return false
		}
	}

	return len(result) == numCourses
}
