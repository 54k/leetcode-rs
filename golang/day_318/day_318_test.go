package day318

import (
	"sort"
	"strings"
)

// https://leetcode.com/problems/remove-duplicate-letters/description
func removeDuplicateLetters(s string) string {
	cnt := make([]int, 26)
	for _, ch := range s {
		cnt[ch-'a']++
	}
	pos := 0
	for i, ch := range s {
		if ch < rune(s[pos]) {
			pos = i
		}
		cnt[rune(s[i])-'a']--
		if cnt[rune(s[i])-'a'] == 0 {
			break
		}
	}
	if len(s) == 0 {
		return ""
	}
	return string(s[pos]) + removeDuplicateLetters(strings.ReplaceAll(s[pos+1:], string(s[pos]), ""))
}

// https://leetcode.com/problems/3sum-smaller/submissions/
func threeSumSmallerBinSearch(nums []int, target int) int {
	search := func(start int, target int) int {
		lo, hi := start, len(nums)-1
		for lo < hi {
			mid := (lo + hi + 1) / 2
			if nums[mid] < target {
				lo = mid
			} else {
				hi = mid - 1
			}
		}
		return lo
	}

	twoSmaller := func(start int, target int) int {
		sum := 0
		for i := start; i < len(nums)-1; i++ {
			sum += search(i, target-nums[i]) - i
		}
		return sum
	}

	sort.Ints(nums)
	sum := 0
	for i := 0; i < len(nums)-2; i++ {
		sum += twoSmaller(i+1, target-nums[i])
	}

	return sum
}

func threeSumSmallerTwoPointers(nums []int, target int) int {
	twoSmaller := func(start int, target int) int {
		sum := 0
		for i, j := start, len(nums)-1; i < j; {
			s := nums[i] + nums[j]
			if s < target {
				sum += j - i
				i++
			} else {
				j--
			}
		}
		return sum
	}

	sort.Ints(nums)
	sum := 0
	for i := 0; i < len(nums); i++ {
		sum += twoSmaller(i+1, target-nums[i])
	}
	return sum
}

// https://leetcode.com/problems/maximum-binary-tree/description/
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func constructMaximumBinaryTree(nums []int) *TreeNode {
	max := func(left, right int) int {
		maxi := left
		for i := left; i <= right; i++ {
			if nums[i] > nums[maxi] {
				maxi = i
			}
		}
		return maxi
	}

	var buildRec func(left, right int) *TreeNode
	buildRec = func(left, right int) *TreeNode {
		if left > right {
			return nil
		}
		maxi := max(left, right)
		root := &TreeNode{nums[maxi], nil, nil}
		root.Left = buildRec(left, maxi-1)
		root.Right = buildRec(maxi+1, right)
		return root
	}
	return buildRec(0, len(nums)-1)
}
