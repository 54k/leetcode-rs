package day537

import "strconv"

// https://leetcode.com/problems/compare-version-numbers/description
func getNextChunk(version string, n int, p int) (int, int) {
	if p > n-1 {
		return 0, p
	}
	pEnd := p
	for pEnd < n && version[pEnd] != '.' {
		pEnd++
	}
	i, _ := strconv.Atoi(version[p:pEnd])
	p = pEnd + 1
	return i, p
}

func compareVersion(version1 string, version2 string) int {
	p1, p2 := 0, 0
	n1, n2 := len(version1), len(version2)
	for p1 < n1 || p2 < n2 {
		i1, p1_ := getNextChunk(version1, n1, p1)
		p1 = p1_
		i2, p2_ := getNextChunk(version2, n2, p2)
		p2 = p2_
		if i1 != i2 {
			if i1 > i2 {
				return 1
			} else {
				return -1
			}
		}
	}
	return 0
}
