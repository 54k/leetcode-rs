package day271

import "fmt"

// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/
func search(nums []int, target int) bool {
	existsInFirst := func(start int, el int) bool {
		return nums[start] <= el
	}
	isBinaryHelpful := func(left int, el int) bool {
		return nums[left] != el
	}
	left, right := 0, len(nums)-1
	for left <= right {
		mid := left + (right-left)/2
		if nums[mid] == target {
			return true
		}
		if !isBinaryHelpful(left, nums[mid]) {
			left++
			continue
		}
		pivotArray := existsInFirst(left, nums[mid])
		targetArray := existsInFirst(left, target)

		if pivotArray != targetArray {
			if pivotArray {
				left = mid + 1
			} else {
				right = mid - 1
			}
		} else {
			if nums[mid] < target {
				left = mid + 1
			} else {
				right = mid - 1
			}
		}
	}
	return false
}

// https://leetcode.com/problems/sort-colors/description/
func sortColors(nums []int) {
	p0 := 0
	curr := 0
	p2 := len(nums) - 1

	var tmp int
	for curr <= p2 {
		if nums[curr] == 0 {
			tmp = nums[p0]
			nums[p0] = nums[curr]
			nums[curr] = tmp
			p0++
			curr++
		} else if nums[curr] == 2 {
			tmp = nums[curr]
			nums[curr] = nums[p2]
			nums[p2] = tmp
			p2--
		} else {
			curr++
		}
	}
}

// https://leetcode.com/problems/subtree-of-another-tree/description/
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isSubtree(root *TreeNode, subRoot *TreeNode) bool {
	var isSame func(*TreeNode, *TreeNode) bool
	isSame = func(l *TreeNode, r *TreeNode) bool {
		if l == nil || r == nil {
			return l == nil && r == nil
		}
		return l.Val == r.Val && isSame(l.Left, r.Left) && isSame(l.Right, r.Right)
	}
	var dfs func(*TreeNode) bool
	dfs = func(root *TreeNode) bool {
		if root == nil {
			return false
		}

		if root.Val == subRoot.Val && isSame(root, subRoot) {
			return true
		}
		return dfs(root.Left) || dfs(root.Right)
	}

	return dfs(root)
}

// https://leetcode.com/problems/find-duplicate-subtrees/description/
func findDuplicateSubtrees(root *TreeNode) []*TreeNode {
	res := []*TreeNode{}
	cnt := map[string]int{}
	var dfs func(*TreeNode) string
	dfs = func(root *TreeNode) string {
		if root == nil {
			return ""
		}
		key := fmt.Sprintf("(%s)%s(%s)", dfs(root.Left), root.Val, dfs(root.Right))
		cnt[key] += 1
		if cnt[key] == 2 {
			res = append(res, root)
		}
		return key
	}
	dfs(root)
	return res
}

// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree-iii/description/
type Node struct {
	Val    int
	Left   *Node
	Right  *Node
	Parent *Node
}

func lowestCommonAncestor(p *Node, q *Node) *Node {
	ancestor := map[*Node]bool{}
	for p != nil {
		ancestor[p] = true
		p = p.Parent
	}
	for q != nil && !ancestor[q] {
		q = q.Parent
	}
	return q
}

// https://leetcode.com/problems/longest-univalue-path/description/
func longestUnivaluePath(root *TreeNode) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	ans := 0
	var arrowLen func(*TreeNode) int
	arrowLen = func(root *TreeNode) int {
		if root == nil {
			return 0
		}
		left := arrowLen(root.Left)
		right := arrowLen(root.Right)
		arrowLeft, arrowRight := 0, 0
		if root.Left != nil && root.Left.Val == root.Val {
			arrowLeft += left + 1
		}
		if root.Right != nil && root.Right.Val == root.Val {
			arrowRight += right + 1
		}
		ans = max(ans, arrowLeft+arrowRight)
		return max(arrowLeft, arrowRight)
	}
	arrowLen(root)
	return ans
}

// https://leetcode.com/problems/remove-duplicate-letters/description/
func removeDuplicateLettersGreedyLetterByLetter(s string) string {
	cnt := make([]int, 26)
	pos := 0
	for i := 0; i < len(s); i++ {
		cnt[s[i]-'a']++
	}

	for i := 0; i < len(s); i++ {
		if s[i] < s[pos] {
			pos = i
		}

		cnt[s[i]-'a']--
		if cnt[s[i]-'a'] == 0 {
			break
		}
	}

	if len(s) == 0 {
		return ""
	}

	suffix := s[pos+1:]
	replaced := ""
	for _, ch := range []byte(suffix) {
		if ch == s[pos] {
			continue
		}
		replaced += string(ch)
	}
	return string(s[pos]) + removeDuplicateLettersGreedyLetterByLetter(replaced)
}

func removeDuplicateLettersWithCounter(s string) string {
	cnt := make([]int, 26)
	for i := 0; i < len(s); i++ {
		cnt[s[i]-'a']++
	}
	inStack := map[rune]bool{}
	stack := []rune{}

	for _, ch := range s {
		cnt[ch-'a']--
		if !inStack[ch] {
			for len(stack) > 0 && cnt[stack[len(stack)-1]-'a'] > 0 && stack[len(stack)-1] > ch {
				top := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				inStack[top] = false
			}
			inStack[ch] = true
			stack = append(stack, ch)
		}
	}

	return string(stack)
}

func removeDuplicateLetters(s string) string {
	lastIdx := make([]int, 26)
	for i := 0; i < len(s); i++ {
		lastIdx[s[i]-'a'] = i
	}
	inStack := map[rune]bool{}
	stack := []rune{}

	for i, ch := range s {
		if !inStack[ch] {
			for len(stack) > 0 && lastIdx[stack[len(stack)-1]-'a'] > i && stack[len(stack)-1] > ch {
				top := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				inStack[top] = false
			}
			inStack[ch] = true
			stack = append(stack, ch)
		}
	}

	return string(stack)
}

// https://leetcode.com/problems/maximize-distance-to-closest-person/description/
func maxDistToClosestArrays(seats []int) int {
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

	leftDist, rightDist := make([]int, len(seats)), make([]int, len(seats))
	for i := 0; i < len(seats); i++ {
		leftDist[i], rightDist[i] = len(seats), len(seats)
	}

	for i := 0; i < len(seats); i++ {
		if seats[i] == 1 {
			leftDist[i] = 0
		} else if i > 0 {
			leftDist[i] = leftDist[i-1] + 1
		}
	}
	for i := len(seats) - 1; i >= 0; i-- {
		if seats[i] == 1 {
			rightDist[i] = 0
		} else if i < len(seats)-1 {
			rightDist[i] = rightDist[i+1] + 1
		}
	}

	ans := 0
	for i := 0; i < len(seats); i++ {
		if seats[i] == 0 {
			ans = max(ans, min(leftDist[i], rightDist[i]))
		}
	}
	return ans
}

func maxDistToClosestTwoPointers(seats []int) int {
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

	prev, future := -1, 0
	ans := 0
	for i := 0; i < len(seats); i++ {
		if seats[i] == 1 {
			prev = i
		} else {
			for future < len(seats) && seats[future] == 0 || future < i {
				future++
			}
			left := len(seats)
			if prev > -1 {
				left = i - prev
			}
			right := len(seats)
			if future < len(seats) {
				right = future - i
			}
			ans = max(ans, min(left, right))
		}
	}
	return ans
}

func maxDistToClosestZeroGroups(seats []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	n := len(seats)
	ans := 0
	k := 0
	for i := 0; i < n; i++ {
		if seats[i] == 1 {
			ans = max(ans, (k+1)/2)
			k = 0
		} else {
			k++
		}
	}

	for i := 0; i < n; i++ {
		if seats[i] == 1 {
			ans = max(ans, i)
			break
		}
	}

	for i := n - 1; i >= 0; i-- {
		if seats[i] == 1 {
			ans = max(ans, n-1-i)
			break
		}
	}
	return ans
}
