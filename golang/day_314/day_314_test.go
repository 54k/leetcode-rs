package day314

// https://leetcode.com/problems/median-of-two-sorted-arrays/description/
func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	if len(nums1) > len(nums2) {
		return findMedianSortedArrays(nums2, nums1)
	}

	m, n := len(nums1), len(nums2)
	half := (m + n + 1) / 2

	left, right := 0, m
	for left <= right {
		partitionA := (left + right) / 2
		partitionB := half - partitionA

		maxLeftA := -(1 << 30)
		if partitionA > 0 {
			maxLeftA = nums1[partitionA-1]
		}
		minRightA := 1 << 30
		if partitionA < m {
			minRightA = nums1[partitionA]
		}

		maxLeftB := -(1 << 30)
		if partitionB > 0 {
			maxLeftB = nums2[partitionB-1]
		}
		minRightB := 1 << 30
		if partitionB < n {
			minRightB = nums2[partitionB]
		}

		if maxLeftA <= minRightB && maxLeftB <= minRightA {
			if (m+n)%2 == 0 {
				return float64(max(maxLeftA, maxLeftB)+min(minRightA, minRightB)) / 2.0
			} else {
				return float64(max(maxLeftA, maxLeftB))
			}
		} else if maxLeftA > minRightB {
			right = partitionA - 1
		} else {
			left = partitionA + 1
		}
	}

	return 0.0
}

// https://leetcode.com/problems/is-subsequence/description
func isSubsequenceDivideAndConquer(s string, t string) bool {
	leftBound, rightBound := len(s), len(t)
	var recIsSibsequeence func(int, int) bool
	recIsSibsequeence = func(leftIndex, rightIndex int) bool {
		if leftIndex == leftBound {
			return true
		}
		if rightIndex == rightBound {
			return false
		}

		if s[leftIndex] == t[rightIndex] {
			leftIndex++
		}
		rightIndex++
		return recIsSibsequeence(leftIndex, rightIndex)
	}

	return recIsSibsequeence(0, 0)
}

// https://leetcode.com/problems/number-of-matching-subsequences/description/
func numMatchingSubseq(s string, words []string) int {
	type node struct {
		word  string
		index int
	}

	buckets := [][]node{}
	for i := 0; i < 26; i++ {
		buckets = append(buckets, []node{})
	}
	for _, w := range words {
		buckets[w[0]-'a'] = append(buckets[w[0]-'a'], node{w, 0})
	}
	ans := 0
	for _, ch := range s {
		bucket := buckets[ch-'a']
		buckets[ch-'a'] = []node{}

		for _, n := range bucket {
			n.index++
			if n.index == len(n.word) {
				ans++
			} else {
				buckets[n.word[n.index]-'a'] = append(buckets[n.word[n.index]-'a'], n)
			}
		}
	}

	return ans
}

// https://leetcode.com/problems/shortest-way-to-form-string/description/
func shortestWay(source string, target string) int {
	sm := map[rune]bool{}
	for _, ch := range source {
		sm[ch] = true
	}
	for _, ch := range target {
		if !sm[ch] {
			return -1
		}
	}

	ans := 0
	i, j := 0, 0
	for i < len(target) {
		if j == 0 {
			ans++
		}
		if target[i] == source[j] {
			i++
		}
		j = (j + 1) % len(source)
	}

	return ans
}
