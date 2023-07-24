package day_254

import "math/rand"
import "strings"

// https://leetcode.com/problems/top-k-frequent-elements/
func topKFrequent(nums []int, k int) []int {
	count := map[int]int{}
	for _, num := range nums {
		count[num]++
	}
	unique := []int{}
	for k, _ := range count {
		unique = append(unique, k)
	}

	swap := func(left, right int) {
		tmp := unique[left]
		unique[left] = unique[right]
		unique[right] = tmp
	}

	partition := func(left int, right int, pivot_idx int) int {
		pivot_freq := count[unique[pivot_idx]]
		swap(pivot_idx, right)
		store_idx := left

		for i := left; i < right; i++ {
			if count[unique[i]] < pivot_freq {
				swap(store_idx, i)
				store_idx++
			}
		}

		swap(store_idx, right)
		return store_idx
	}

	var quickselect func(int, int, int)
	quickselect = func(left int, right int, k_smallest int) {
		if left == right {
			return
		}
		pivot_idx := left + rand.Intn(right-left)
		pivot_idx = partition(left, right, pivot_idx)

		if k_smallest == pivot_idx {
			return
		} else if k_smallest < pivot_idx {
			quickselect(left, pivot_idx-1, k_smallest)
		} else {
			quickselect(pivot_idx+1, right, k_smallest)
		}
	}

	n := len(unique)

	quickselect(0, n-1, n-k)

	return unique[n-k : n]
}

// https://leetcode.com/problems/course-schedule-ii/description/
func findOrderDFS(numCourses int, prerequisites [][]int) []int {
	adj := map[int][]int{}
	for i := 0; i < numCourses; i++ {
		adj[i] = []int{}
	}
	for _, p := range prerequisites {
		adj[p[1]] = append(adj[p[1]], p[0])
	}
	vis := make([]int, numCourses)
	ans := []int{}

	var dfs func(int) bool
	dfs = func(v int) bool {
		if vis[v] == 1 {
			return true
		}
		vis[v] = 1
		for _, u := range adj[v] {
			if vis[u] != 2 && dfs(u) {
				return true
			}
		}
		vis[v] = 2
		ans = append(ans, v)
		return false
	}

	for i := 0; i < numCourses; i++ {
		if vis[i] == 0 && dfs(i) {
			return []int{}
		}
	}

	for i := 0; i < len(ans)/2; i++ {
		tmp := ans[i]
		ans[i] = ans[len(ans)-1-i]
		ans[len(ans)-1-i] = tmp
	}
	return ans
}

func findOrderKhan(numCourses int, prerequisites [][]int) []int {
	adj := map[int][]int{}
	for _, p := range prerequisites {
		if _, ok := adj[p[0]]; !ok {
			adj[p[0]] = []int{}
		}
		adj[p[1]] = append(adj[p[1]], p[0])
	}

	inDegree := make([]int, numCourses)
	for _, p := range prerequisites {
		inDegree[p[0]]++
	}

	queue := []int{}
	for i, d := range inDegree {
		if d == 0 {
			queue = append(queue, i)
		}
	}
	ans := []int{}
	for len(queue) > 0 {
		v := queue[0]
		queue = queue[1:]
		ans = append(ans, v)

		for _, u := range adj[v] {
			inDegree[u]--
			if inDegree[u] == 0 {
				queue = append(queue, u)
			}
		}
	}

	if len(ans) != numCourses {
		return []int{}
	}

	return ans
}

// https://leetcode.com/problems/alien-dictionary/description/
func alienOrder(words []string) string {
	adj := map[byte][]byte{}

	for i := 0; i < len(words)-1; i++ {
		w1, w2 := words[i], words[i+1]
		l1, l2 := len(w1), len(w2)
		if l1 > l2 && strings.HasPrefix(w1, w2) {
			return ""
		}

		m := l1
		if l2 < m {
			m = l2
		}

		for j := 0; j < m; j++ {
			if w1[j] != w2[j] {
				adj[w2[j]] = append(adj[w2[j]], w1[j])
				break
			}
		}
	}

	path := []byte{}
	vis := make([]byte, 255)

	var dfs func(byte) bool
	dfs = func(v byte) bool {
		if vis[v] == 1 {
			return true
		}
		vis[v] = 1
		for _, u := range adj[v] {
			if vis[u] != 2 && dfs(u) {
				return true
			}
		}
		vis[v] = 2
		path = append(path, v)
		return false
	}

	letters := map[byte]bool{}

	for _, word := range words {
		for _, letter := range word {
			letters[byte(letter)] = true
		}
	}

	for k, _ := range letters {
		if vis[k] == 0 && dfs(k) {
			return ""
		}
	}

	return string(path)
}
