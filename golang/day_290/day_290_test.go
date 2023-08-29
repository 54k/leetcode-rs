package day290

// https://leetcode.com/problems/minimum-penalty-for-a-shop/description/
func bestClosingTime(customers string) int {
	curPenalty := 0
	minPenalty := 0
	closingTime := 0

	for i, ch := range customers {
		if ch == 'Y' {
			curPenalty -= 1
		} else {
			curPenalty += 1
		}

		if curPenalty < minPenalty {
			minPenalty = curPenalty
			closingTime = i + 1
		}
	}
	return closingTime
}

// https://leetcode.com/problems/two-sum-bsts/
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func twoSumBSTs(root1 *TreeNode, root2 *TreeNode, target int) bool {
	if root1 == nil || root2 == nil {
		return false
	}

	var find func(val int, root *TreeNode) bool
	find = func(val int, root *TreeNode) bool {
		if root == nil {
			return false
		}

		v := root.Val
		if val < v {
			return find(val, root.Left)
		} else if val > v {
			return find(val, root.Right)
		}
		return val == v
	}

	stack := []*TreeNode{root1}
	for len(stack) > 0 {
		pop := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		if find(target-pop.Val, root2) {
			return true
		}

		if pop.Left != nil {
			stack = append(stack, pop.Left)
		}

		if pop.Right != nil {
			stack = append(stack, pop.Right)
		}
	}

	return false
}

func twoSumBSTsTwoSets(root1 *TreeNode, root2 *TreeNode, target int) bool {
	var dfs func(root *TreeNode, set map[int]bool)
	dfs = func(root *TreeNode, set map[int]bool) {
		if root == nil {
			return
		}
		set[root.Val] = true
		dfs(root.Left, set)
		dfs(root.Right, set)
	}

	set1 := make(map[int]bool)
	set2 := make(map[int]bool)
	dfs(root1, set1)
	dfs(root2, set2)

	for n1, _ := range set1 {
		if _, ok := set2[target-n1]; ok {
			return true
		}
	}

	return false
}

func twoSumBSTsTwoPointers(root1 *TreeNode, root2 *TreeNode, target int) bool {
	if root1 == nil || root2 == nil {
		return false
	}

	list1 := []int{}
	list2 := []int{}

	var dfs func(*TreeNode, *[]int)
	dfs = func(tn *TreeNode, list *[]int) {
		if tn == nil {
			return
		}
		dfs(tn.Left, list)
		*list = append(*list, tn.Val)
		dfs(tn.Right, list)
	}

	dfs(root1, &list1)
	dfs(root2, &list2)

	left := 0
	right := len(list2) - 1
	for left < len(list1) && right >= 0 {
		sum := list1[left] + list2[right]
		if sum == target {
			return true
		} else if sum < target {
			left++
		} else {
			right--
		}
	}
	return false
}

type MorrisIterator struct {
	current *TreeNode
	pre     *TreeNode
}

func morrisConstructor(root *TreeNode) *MorrisIterator { return &MorrisIterator{root, nil} }

func (this *MorrisIterator) next() int {
	var val int

	for this.current != nil {
		if this.current.Left == nil {
			val = this.current.Val
			this.current = this.current.Right
			break
		} else {
			this.pre = this.current.Left
			for this.pre.Right != nil && this.pre.Right != this.current {
				this.pre = this.pre.Right
			}

			if this.pre.Right == nil {
				this.pre.Right = this.current
				this.current = this.current.Left
			} else {
				this.pre.Right = nil
				val = this.current.Val
				this.current = this.current.Right
				break
			}
		}
	}

	return val
}

func (this *MorrisIterator) hasNext() bool { return this.current != nil }

type ReverseMorrisIterator struct {
	current *TreeNode
	pre     *TreeNode
}

func reverseMorrisConstructor(root *TreeNode) *ReverseMorrisIterator {
	return &ReverseMorrisIterator{root, nil}
}

func (this *ReverseMorrisIterator) next() int {
	var val int

	for this.current != nil {
		if this.current.Right == nil {
			val = this.current.Val
			this.current = this.current.Left
			break
		} else {
			this.pre = this.current.Right
			for this.pre.Left != nil && this.pre.Left != this.current {
				this.pre = this.pre.Left
			}

			if this.pre.Left == nil {
				this.pre.Left = this.current
				this.current = this.current.Right
			} else {
				this.pre.Left = nil
				val = this.current.Val
				this.current = this.current.Left
				break
			}
		}
	}

	return val
}

func (this *ReverseMorrisIterator) hasNext() bool { return this.current != nil }

func twoSumBSTsMorrisTraversal(root1 *TreeNode, root2 *TreeNode, target int) bool {
	const MIN = -(1 << 31)

	it1 := morrisConstructor(root1)
	it2 := reverseMorrisConstructor(root2)

	left := it1.next()
	right := it2.next()

	for left != MIN && right != MIN {
		sum := left + right

		if sum == target {
			return true
		} else if sum > target {
			if it2.hasNext() {
				right = it2.next()
			} else {
				right = MIN
			}
		} else {
			if it1.hasNext() {
				left = it1.next()
			} else {
				left = MIN
			}
		}
	}

	return false
}

// https://leetcode.com/problems/remove-nodes-from-linked-list/description/“
type ListNode struct {
	Val  int
	Next *ListNode
}

func removeNodes(head *ListNode) *ListNode {
	stack := []*ListNode{}
	node := head
	for node.Next != nil {
		stack = append(stack, node)
		node = node.Next
	}

	max := -(1 << 31)
	var newHead *ListNode

	for i := len(stack) - 1; i >= 0; i-- {
		cur := stack[i]
		if cur.Val >= max {
			max = cur.Val
			cur.Next = newHead
			newHead = cur
		}
	}

	return newHead
}

func removeNodesRec(head *ListNode) *ListNode {
	var rec func(*ListNode) (*ListNode, int)

	rec = func(ln *ListNode) (*ListNode, int) {
		if ln == nil {
			return nil, -(1 << 31)
		}
		prev, max := rec(ln.Next)
		if ln.Val < max {
			return prev, max
		} else {
			ln.Next = prev
			if ln.Val > max {
				return ln, ln.Val
			}
			return ln, max
		}
	}

	res, _ := rec(head)
	return res
}
