package day298

import (
	"fmt"
	"sort"
	"testing"
)

// https://leetcode.com/problems/split-linked-list-in-parts/description/
type ListNode struct {
	Val  int
	Next *ListNode
}

func splitListToPartsCreateNewList(head *ListNode, k int) []*ListNode {
	cur := head
	n := 0
	for ; cur != nil; n++ {
		cur = cur.Next
	}

	width := n / k
	rem := n % k

	ans := make([]*ListNode, k)
	cur = head

	for i := 0; i < k; i++ {
		root := &ListNode{0, nil}
		write := root
		endWidth := width
		if i < rem {
			endWidth++
		}

		for j := 0; j < endWidth; j++ {
			write.Next = &ListNode{cur.Val, nil}
			write = write.Next
			if cur != nil {
				cur = cur.Next
			}
		}
		ans[i] = root.Next
	}

	return ans
}

func splitListToPartsSplitCurrentList(head *ListNode, k int) []*ListNode {
	len := 0
	next := head
	for ; next != nil; len++ {
		next = next.Next
	}

	ans := make([]*ListNode, k)
	width := len / k
	rem := len % k

	cur := head
	for i := 0; i < k; i++ {
		end := width
		if i < rem {
			end++
		}
		root := cur
		ans[i] = head
		for j := 0; j < end-1; j++ {
			if cur != nil {
				cur = cur.Next
			}
		}
		if cur != nil {
			prev := cur
			cur = cur.Next
			prev.Next = nil
		}

		ans[i] = root
	}

	return ans
}

// https://leetcode.com/problems/sort-an-array/description/
func sortArrayCountingSort(nums []int) []int {
	abs := func(a int) int {
		if a < 0 {
			return -a
		}
		return a
	}
	bucketSort := func(nums []int, place int) {
		buckets := make([][]int, 10)
		for i := 0; i < len(buckets); i++ {
			buckets[i] = []int{}
		}

		for _, n := range nums {
			digit := abs(n) / place
			digit %= 10
			buckets[digit] = append(buckets[digit], n)
		}

		i := 0
		for _, b := range buckets {
			for _, n := range b {
				nums[i] = n
				i++
			}
		}
	}

	maxi := nums[0]
	for _, n := range nums {
		if maxi < n {
			maxi = n
		}
	}

	totalDigits := 0
	for ; maxi > 0; totalDigits++ {
		maxi /= 10
	}

	for place := 1; totalDigits >= 0; place *= 10 {
		bucketSort(nums, place)
		totalDigits--
	}

	negs := []int{}
	posi := []int{}
	for _, n := range nums {
		if n < 0 {
			negs = append(negs, n)
		} else {
			posi = append(posi, n)
		}
	}

	for i := 0; i < len(negs)/2; i++ {
		negs[i], negs[len(negs)-1-i] = negs[len(negs)-1-i], negs[i]
	}

	i := 0
	for _, n := range negs {
		nums[i] = n
		i++
	}

	for _, n := range posi {
		nums[i] = n
		i++
	}

	return nums
}

func TestSort(t *testing.T) {
	fmt.Println(sortArrayCountingSort([]int{-1, 2, -8, -10}))
}

// https://leetcode.com/problems/sort-an-array/
func longestCommonSubsequenceTopDown(text1 string, text2 string) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	memo := make([][]int, len(text1)+1)
	for i := 0; i < len(text1); i++ {
		memo[i] = make([]int, len(text2)+1)
		for j := 0; j < len(text2); j++ {
			memo[i][j] = -1
		}
	}

	var dp func(int, int) int
	dp = func(i, j int) int {
		if i == len(text1) || j == len(text2) {
			return 0
		}
		if memo[i][j] != -1 {
			return memo[i][j]
		}
		ans := 0
		if text1[i] == text2[j] {
			ans = 1 + dp(i+1, j+1)
		} else {
			ans = max(ans, max(dp(i+1, j), dp(i, j+1)))
		}
		memo[i][j] = ans
		return ans
	}

	return dp(0, 0)
}

func longestCommonSubsequenceBottomUp(text1 string, text2 string) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	dp := make([][]int, len(text1)+1)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, len(text2)+2)
	}

	for i := 1; i <= len(text1); i++ {
		for j := 1; j <= len(text2); j++ {
			k, x := i-1, j-1
			if text1[k] == text2[x] {
				dp[i][j] = 1 + dp[i-1][j-1]
			} else {
				dp[i][j] = max(dp[i-1][j], dp[i][j-1])
			}
		}
	}

	return dp[len(text1)][len(text2)]
}

func longestCommonSubsequenceBottomUpOptimized(text1 string, text2 string) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	if len(text1) > len(text2) {
		text1, text2 = text2, text1
	}

	prev := make([]int, len(text1)+1)
	for col := len(text2) - 1; col >= 0; col-- {
		current := make([]int, len(text1)+1)
		for row := len(text1) - 1; row >= 0; row-- {
			if text1[row] == text2[col] {
				current[row] = 1 + prev[row+1]
			} else {
				current[row] = max(prev[row], current[row+1])
			}
		}
		prev = current
	}

	return prev[0]
}

// https://leetcode.com/problems/sort-an-array/
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func minimumOperations(root *TreeNode) int {
	swaps := 0
	lvl := []*TreeNode{root}
	for len(lvl) > 0 {
		sorted := []int{}
		idxMap := map[int]int{}
		visited := []bool{}

		for i := 0; i < len(lvl); i++ {
			idxMap[lvl[i].Val] = i
			sorted = append(sorted, lvl[i].Val)
			visited = append(visited, false)
		}
		sort.Ints(sorted)

		for i := 0; i < len(sorted); i++ {
			if visited[i] || idxMap[sorted[i]] == i {
				continue
			}

			cycle_size := 0
			j := i
			for !visited[j] {
				visited[j] = true
				cycle_size++
				j = idxMap[sorted[j]]
			}
			if cycle_size > 0 {
				swaps += (cycle_size - 1)
			}
		}

		next := []*TreeNode{}
		for _, e := range lvl {
			if e.Left != nil {
				next = append(next, e.Left)
			}
			if e.Right != nil {
				next = append(next, e.Right)
			}
		}
		lvl = next
	}

	return swaps
}
