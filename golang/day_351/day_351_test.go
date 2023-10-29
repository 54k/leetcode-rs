package day351

import (
	"sort"
)

// https://leetcode.com/problems/meeting-rooms-ii/
func minMeetingRooms(intervals [][]int) int {
	start := make([]int, len(intervals))
	end := make([]int, len(intervals))

	for i, interval := range intervals {
		start[i] = interval[0]
		end[i] = interval[1]
	}

	sort.Ints(start)
	sort.Ints(end)
	sPtr, ePtr := 0, 0
	roomsUsed := 0

	for sPtr < len(start) {
		if start[sPtr] >= end[ePtr] {
			roomsUsed--
			ePtr++
		}

		sPtr++
		roomsUsed++
	}

	return roomsUsed
}

// https://leetcode.com/problems/car-pooling/description/
func carPooling(trips [][]int, capacity int) bool {
	farthest := 0
	for _, trip := range trips {
		if trip[2] > farthest {
			farthest = trip[2]
		}
	}
	numLine := make([]int, farthest+1)
	for _, trip := range trips {
		numLine[trip[1]] += trip[0]
		numLine[trip[2]] -= trip[0]
	}

	for i := 0; i < len(numLine); i++ {
		if i > 0 {
			numLine[i] += numLine[i-1]
		}
		if numLine[i] > capacity {
			return false
		}
	}
	return true
}

// https://leetcode.com/problems/count-positions-on-street-with-required-brightness/description/
func meetRequirement(n int, lights [][]int, requirement []int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	prefix := make([]int, len(lights))
	for _, l := range lights {
		left := max(0, l[0]-l[1])
		right := min(n-1, l[0]+l[1]+1)

		prefix[left] += 1
		prefix[right] -= 1
	}

	for i := 1; i < len(prefix); i++ {
		prefix[i] += prefix[i-1]
	}

	ans := 0
	for i, req := range requirement {
		if prefix[i] >= req {
			ans++
		}
	}

	return ans
}
