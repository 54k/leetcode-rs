package day306

import (
	"fmt"
	"sort"
	"testing"
)

func findItineraryBacktracking(tickets [][]string) []string {
	res := []string{}
	adj := map[string][]string{}
	for _, ticket := range tickets {
		from, to := ticket[0], ticket[1]
		if _, ok := adj[from]; !ok {
			adj[from] = []string{}
		}
		adj[from] = append(adj[from], to)
	}

	vistiBitmap := map[string][]bool{}
	for k, v := range adj {
		sort.Strings(v)
		vistiBitmap[k] = make([]bool, len(v))
	}

	flights := len(tickets)
	var backtrack func(string, []string) bool
	backtrack = func(from string, route []string) bool {
		if len(route) == flights+1 {
			r := make([]string, len(route))
			copy(r, route)
			res = r
			return true
		}

		if _, ok := adj[from]; !ok {
			return false
		}

		i := 0
		bitmap := vistiBitmap[from]
		for _, dest := range adj[from] {
			if !bitmap[i] {
				bitmap[i] = true
				route = append(route, dest)
				ret := backtrack(dest, route)
				route = route[:len(route)-1]
				bitmap[i] = false
				if ret {
					return true
				}
			}
			i++
		}
		return false
	}

	backtrack("JFK", []string{"JFK"})
	return res
}

func findItinerary(tickets [][]string) []string {
	res := []string{}
	adj := map[string][]string{}
	for _, ticket := range tickets {
		from, to := ticket[0], ticket[1]
		if _, ok := adj[from]; !ok {
			adj[from] = []string{}
		}
		adj[from] = append(adj[from], to)
	}

	for _, v := range adj {
		sort.Strings(v)
	}

	visited := map[string]int{}

	var dfs func(string)
	dfs = func(from string) {
		if _, ok := adj[from]; ok {
			for visited[from] != len(adj[from]) {
				to := adj[from][visited[from]]
				visited[from]++
				dfs(to)
			}
		}

		res = append(res, from)
	}

	dfs("JFK")

	for i := 0; i < len(res)/2; i++ {
		res[i], res[len(res)-1-i] = res[len(res)-1-i], res[i]
	}

	return res
}

func TestFindItinerary(t *testing.T) {
	res := findItinerary([][]string{{"JFK", "SFO"}, {"JFK", "ATL"}, {"SFO", "ATL"}, {"ATL", "JFK"}, {"ATL", "SFO"}})
	fmt.Println(res)
}
