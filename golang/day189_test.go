package main

import (
	"fmt"
	"testing"
)

type entry struct {
	v     string
	ratio float64
}

func calcEquation(equations [][]string, values []float64, queries [][]string) []float64 {
	graph := map[string]map[string]float64{}

	search := func(start, div string) float64 {
		if _, ok := graph[start]; !ok {
			return -1.
		}
		seen := map[string]bool{start: true}
		stack := []entry{{v: start, ratio: 1.}}

		for len(stack) > 0 {
			pop := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			if pop.v == div {
				return pop.ratio
			}

			for u, num := range graph[pop.v] {
				if !seen[u] {
					seen[u] = true
					stack = append(stack, entry{v: u, ratio: pop.ratio * num})
				}
			}

		}

		return -1.
	}

	for i, v := range equations {
		num, div := v[0], v[1]
		if _, ok := graph[num]; !ok {
			graph[num] = map[string]float64{}
		}
		graph[num][div] = values[i]

		if _, ok := graph[div]; !ok {
			graph[div] = map[string]float64{}
		}
		graph[div][num] = 1. / values[i]

	}

	ans := []float64{}
	for _, q := range queries {
		ans = append(ans, search(q[0], q[1]))
	}

	return ans
}

// https://leetcode.com/problems/sum-of-subarray-minimums/description/
func sumSubarrayMins(arr []int) int {
	const MOD = 1000000007
	stack := []int{}
	sum := 0

	for right := 0; right <= len(arr); right++ {
		for len(stack) > 0 && (right == len(arr) || arr[stack[len(stack)-1]] >= arr[right]) {
			mini := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			left := -1
			if len(stack) > 0 {
				left = stack[len(stack)-1]
			}
			r := (mini - left) * (right - mini) % MOD
			s := (arr[mini] * r) % MOD
			sum += s
			sum %= MOD
		}

		stack = append(stack, right)
	}

	return sum
}

// https://leetcode.com/problems/sum-of-subarray-ranges/description/
func subArrayRanges(arr []int) int64 {
	stack := []int{}
	diff := int64(0)
	for right := 0; right <= len(arr); right++ {
		for len(stack) > 0 && (right == len(arr) || arr[stack[len(stack)-1]] >= arr[right]) {
			mini := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			left := -1
			if len(stack) > 0 {
				left = stack[len(stack)-1]
			}

			diff -= int64(arr[mini] * (mini - left) * (right - mini))
		}
		stack = append(stack, right)
	}
	stack = []int{}

	for right := 0; right <= len(arr); right++ {
		for len(stack) > 0 && (right == len(arr) || arr[stack[len(stack)-1]] <= arr[right]) {
			mini := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			left := -1
			if len(stack) > 0 {
				left = stack[len(stack)-1]
			}

			diff += int64(arr[mini] * (mini - left) * (right - mini))
		}
		stack = append(stack, right)
	}
	return diff
}

func TestCalcEquation(t *testing.T) {
	fmt.Println(calcEquation([][]string{{"a", "b"}, {"b", "c"}}, []float64{2., 3.}, [][]string{{"b", "a"}}))
}
