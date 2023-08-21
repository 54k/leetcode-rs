package day282

import (
	"sort"
	"strconv"
	"strings"
)

// https://leetcode.com/problems/repeated-substring-pattern/description/
func repeatedSubstringPattern(s string) bool {
	if len(s) == 1 {
		return false
	}
	isOk := func(pat string) bool {
		s2 := ""
		for i := 0; i < len(s)/len(pat); i++ {
			s2 += pat
		}
		return s == s2
	}

	for i := 1; i <= len(s)/2; i++ {
		if len(s)%i == 0 && isOk(s[:i]) {
			return true
		}
	}
	return false
}

func repeatedSubstringPattern2(s string) bool {
	t := s + s
	return strings.Contains(t[1:len(t)-1], s)
}

// https://leetcode.com/problems/find-right-interval/description/
func findRightInterval(intervals [][]int) []int {
	ktos := func(key []int) string {
		res := ""
		for _, e := range key {
			res += strconv.Itoa(e) + ","
		}
		return res
	}

	res := make([]int, len(intervals))

	hash := map[string]int{}
	for i, interval := range intervals {
		hash[ktos(interval)] = i
	}

	sort.Slice(intervals, func(a, b int) bool {
		return intervals[a][0] < intervals[b][0]
	})

	var binSearch func(int, int, int) []int
	binSearch = func(target int, start int, end int) []int {
		// if start >= end {
		//     if intervals[start][0] >= target {
		//         return intervals[start]
		//     }
		//     return nil
		// }

		// mid := (start + end) / 2
		// if intervals[mid][0] < target {
		//     return binSearch(target, mid + 1, end)
		// } else {
		//     return binSearch(target, start, mid)
		// }
		lo, hi := start, end
		for lo < hi {
			mid := (lo + hi) / 2
			if intervals[mid][0] < target {
				lo = mid + 1
			} else {
				hi = mid
			}
		}
		if intervals[lo][0] < target {
			return nil
		}
		return intervals[lo]
	}

	for _, interval := range intervals {
		interval2 := binSearch(interval[1], 0, len(intervals)-1)
		res[hash[ktos(interval)]] = -1
		if interval2 != nil {
			res[hash[ktos(interval)]] = hash[ktos(interval2)]
		}
	}
	return res
}
