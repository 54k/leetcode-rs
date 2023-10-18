package day340

import (
	"fmt"
	"testing"
	"unicode"
)

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

// 1 - 2 * 3 / 2 == -2
// https://leetcode.com/problems/basic-calculator-ii/description/
func calculate(s string) int {
	result := 0

	currentNum := 0
	operator := '+'
	lastNum := 0

	for i, ch := range s {
		if unicode.IsDigit(ch) {
			currentNum = currentNum*10 + int(ch-'0')
		}

		if !unicode.IsDigit(ch) && ch != ' ' || i == len(s)-1 {
			if operator == '+' {
				result += lastNum
				lastNum = currentNum
			} else if operator == '-' {
				result += lastNum
				lastNum = -currentNum
			} else if operator == '*' {
				lastNum *= currentNum
			} else if operator == '/' {
				lastNum /= currentNum
			}

			operator = ch
			currentNum = 0
		}
	}

	return result + lastNum
}

func TestCalclulate(t *testing.T) {
	res := calculate("1 - 2 * 3 / 2")
	fmt.Println(res) // -2
}
