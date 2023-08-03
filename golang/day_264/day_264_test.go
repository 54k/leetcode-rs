package day264

// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
func letterCombinations(digits string) []string {
	mapping := map[rune][]rune{}
	mapping['2'] = []rune("abc")
	mapping['3'] = []rune("def")
	mapping['4'] = []rune("ghi")
	mapping['5'] = []rune("jkl")
	mapping['6'] = []rune("mno")
	mapping['7'] = []rune("pqrs")
	mapping['8'] = []rune("tuv")
	mapping['9'] = []rune("wxyz")

	ans := []string{}
	cur := []rune{}

	var rec func(int)
	rec = func(i int) {
		if i == len(digits) {
			dst := ""
			if string(cur) == dst {
				return
			}
			for _, ch := range cur {
				dst += string(ch)
			}
			ans = append(ans, dst)
			return
		}

		for _, ch := range mapping[rune(digits[i])] {
			cur = append(cur, ch)
			rec(i + 1)
			cur = cur[:len(cur)-1]
		}

	}

	rec(0)

	return ans
}

// https://leetcode.com/problems/design-linked-list/description/
type Node struct {
	val  int
	next *Node
}

type MyLinkedList struct {
	head *Node
}

func Constructor() MyLinkedList {
	return MyLinkedList{nil}
}

func (this *MyLinkedList) getNode(index int) *Node {
	cur := this.head
	for cur != nil && index > 0 {
		cur = cur.next
		index--
	}
	return cur
}

func (this *MyLinkedList) getTail() *Node {
	cur := this.head
	for cur != nil && cur.next != nil {
		cur = cur.next
	}
	return cur
}

func (this *MyLinkedList) Get(index int) int {
	cur := this.getNode(index)
	if cur == nil {
		return -1
	}
	return cur.val
}

func (this *MyLinkedList) AddAtHead(val int) {
	this.head = &Node{val, this.head}
}

func (this *MyLinkedList) AddAtTail(val int) {
	if this.head == nil {
		this.AddAtHead(val)
		return
	}
	tail := this.getTail()
	tail.next = &Node{val, nil}
}

func (this *MyLinkedList) AddAtIndex(index int, val int) {
	if index == 0 {
		this.AddAtHead(val)
		return
	}
	prev := this.getNode(index - 1)
	if prev == nil {
		return
	}
	cur := &Node{val, prev.next}
	prev.next = cur
}

func (this *MyLinkedList) DeleteAtIndex(index int) {
	cur := this.getNode(index)
	if cur == nil {
		return
	}
	if index == 0 {
		this.head = cur.next
	} else {
		prev := this.getNode(index - 1)
		prev.next = cur.next
	}
}

// https://leetcode.com/problems/linked-list-cycle/description/
type ListNode struct {
	Val  int
	Next *ListNode
}

func hasCycleFloyd1(head *ListNode) bool {
	fast, slow := head, head

	for fast != nil && fast.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next

		if slow == fast {
			return true
		}
	}

	return false
}

func hasCycleFloyd2(head *ListNode) bool {
	if head == nil {
		return false
	}
	fast, slow := head.Next, head
	for slow != fast {
		if fast == nil || fast.Next == nil {
			return false
		}
		slow = slow.Next
		fast = fast.Next.Next
	}
	return true
}

// https://leetcode.com/problems/linked-list-cycle-ii/description/
func detectCycleFloyd1(head *ListNode) *ListNode {
	slow, fast := head, head
	for fast != nil && fast.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next

		if slow == fast {
			slow = head
			for slow != fast {
				slow = slow.Next
				fast = fast.Next
			}
			return slow
		}
	}
	return nil
}

// https://leetcode.com/problems/intersection-of-two-linked-lists/description/
func getIntersectionNode1(headA, headB *ListNode) *ListNode {
	len := func(h *ListNode) int {
		ans := 0
		for h != nil {
			h = h.Next
			ans++
		}
		return ans
	}

	diff := len(headA) - len(headB)
	if diff < 0 {
		diff *= -1
	} else {
		headA, headB = headB, headA
	}

	for diff > 0 {
		headB = headB.Next
		diff--
	}

	for headA != nil && headB != nil && headA != headB {
		headA = headA.Next
		headB = headB.Next
	}
	return headA
}

func getIntersectionNode2(headA, headB *ListNode) *ListNode {
	pA, pB := headA, headB
	for pA != pB {
		if pA == nil {
			pA = headB
		} else {
			pA = pA.Next
		}

		if pB == nil {
			pB = headA
		} else {
			pB = pB.Next
		}
	}

	return pA
}

// https://leetcode.com/problems/remove-nth-node-from-end-of-list/description/
func removeNthFromEnd(head *ListNode, n int) *ListNode {
	dummy := &ListNode{0, head}
	first, second := dummy, dummy

	for i := 0; i <= n; i++ {
		first = first.Next
	}

	for first != nil {
		first = first.Next
		second = second.Next
	}

	second.Next = second.Next.Next
	return dummy.Next
}

// https://leetcode.com/problems/find-k-length-substrings-with-no-repeated-characters/description/
func numKLenSubstrNoRepeats(s string, k int) int {
	if k > 26 {
		return 0
	}

	freq := make([]int, 26)
	n := len(s)
	left := 0
	ans := 0
	for right := 0; right < n; right++ {
		freq[s[right]-'a']++

		for freq[s[right]-'a'] > 1 {
			freq[s[left]-'a']--
			left++
		}

		if right-left+1 == k {
			ans++
			freq[s[left]-'a']--
			left++
		}
	}
	return ans
}

// https://leetcode.com/problems/binary-tree-maximum-path-sum/description/
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func maxPathSum(root *TreeNode) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	ans := -(1 << 30)
	var dfs func(*TreeNode) int
	dfs = func(root *TreeNode) int {
		if root == nil {
			return 0
		}

		left := max(dfs(root.Left), 0)
		right := max(dfs(root.Right), 0)

		total := root.Val + left + right

		ans = max(ans, total)

		return max(root.Val+left, root.Val+right)
	}
	dfs(root)
	return ans
}

// https://leetcode.com/problems/extra-characters-in-a-string/description/
func minExtraChar(s string, dictionary []string) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	memo := make([]int, len(s))
	for i := 0; i < len(memo); i++ {
		memo[i] = -1
	}
	dict := map[string]bool{}
	for _, w := range dictionary {
		dict[w] = true
	}

	var dfs func(int) int
	dfs = func(start int) int {
		if start == len(s) {
			return 0
		}
		if memo[start] != -1 {
			return memo[start]
		}
		ans := dfs(start+1) + 1
		for end := start; end < len(s); end++ {
			for w, _ := range dict {
				cur := string(s[start : end+1])
				if w == cur {
					ans = min(ans, dfs(end+1))
				}
			}
		}
		memo[start] = ans
		return ans
	}

	return dfs(0)
}
