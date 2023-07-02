package main

import (
	"fmt"
	"testing"
)

// https://leetcode.com/problems/maximum-number-of-achievable-transfer-requests/
func maximumRequests(n int, requests [][]int) int {
	popCount := func(num int) int {
		res := 0
		for ; num > 0; num >>= 1 {
			res += num & 1
		}
		return res
	}

	answer := 0
label:
	for mask := 0; mask < 1<<len(requests); mask++ {
		// fmt.Printf("%08b\n", mask)
		indegree := make([]int, n)

		bitCount := popCount(mask)
		if bitCount <= answer {
			continue label
		}

		for curr, pos := mask, len(requests)-1; curr > 0; curr = curr >> 1 {
			if curr&1 == 1 {
				indegree[requests[pos][0]]--
				indegree[requests[pos][1]]++
			}
			pos--
		}

		for _, d := range indegree {
			if d != 0 {
				continue label
			}
		}
		if bitCount > answer {
			answer = bitCount
		}
	}
	return answer
}

func TestMaxRequests(t *testing.T) {
	res := maximumRequests(5, [][]int{{0, 1}, {1, 0}, {0, 1}, {1, 2}, {2, 0}, {3, 4}})
	fmt.Println("res: ", res)
}
