package main

import (
	"fmt"
	"sort"
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

// https://leetcode.com/problems/maximum-white-tiles-covered-by-a-carpet/description/
func maximumWhiteTiles(tiles [][]int, carpetLen int) int {
	sort.SliceStable(tiles, func(i, j int) bool {
		return tiles[i][0] < tiles[j][0]
	})
	i, j := 0, 0
	cover := 0
	answer := 0
	for i < len(tiles) && answer < carpetLen {
		if tiles[j][0]+carpetLen > tiles[i][1] {
			cover += tiles[i][1] - tiles[i][0] + 1
			if cover > answer {
				answer = cover
			}
			i++
		} else {
			partial := tiles[j][0] + carpetLen - tiles[i][0]
			if partial < 0 {
				partial = 0
			}
			if cover+partial > answer {
				answer = cover + partial
			}
			cover -= tiles[j][1] - tiles[j][0] + 1
			j++
		}
	}
	return answer
}
