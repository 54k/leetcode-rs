package day265

// https://leetcode.com/problems/word-break/description/
func wordBreak(s string, wordDict []string) bool {
	dp := make([]bool, len(s))
	dict := map[string]bool{}
	for _, w := range wordDict {
		dict[w] = true
	}
	for i := 0; i < len(s); i++ {
		for w, _ := range dict {
			if i >= len(w)-1 && (i == len(w)-1 || dp[i-len(w)]) {
				cur := s[i-len(w)+1 : i+1]
				if w == cur {
					dp[i] = true
					break
				}
			}
		}
	}
	return dp[len(s)-1]
}

// https://leetcode.com/problems/remove-linked-list-elements/description/
type ListNode struct {
	Val  int
	Next *ListNode
}

func removeElements(head *ListNode, val int) *ListNode {
	dummy := &ListNode{0, head}
	it := dummy
	for it != nil && it.Next != nil {
		if it.Next.Val == val {
			it.Next = it.Next.Next
		} else {
			it = it.Next
		}
	}
	return dummy.Next
}

// https://leetcode.com/problems/odd-even-linked-list/submissions/
func oddEvenList(n *ListNode) *ListNode {
	head := &ListNode{0, nil}
	headFirst := head

	tail := &ListNode{0, nil}
	tailFirst := tail

	idx := 1
	for n != nil {
		next := n.Next
		n.Next = nil

		if idx%2 == 1 {
			head.Next = n
			head = head.Next
		} else {
			tail.Next = n
			tail = tail.Next
		}
		idx++
		n = next
	}

	head.Next = tailFirst.Next
	return headFirst.Next
}

// https://leetcode.com/problems/odd-even-linked-list/description/
func oddEvenListLeetcodeSolution(head *ListNode) *ListNode {
	if head == nil {
		return head
	}
	odd, even := head, head.Next
	evenHead := even

	for even != nil && even.Next != nil {
		odd.Next = even.Next
		odd = odd.Next
		even.Next = odd.Next
		even = even.Next
	}

	odd.Next = evenHead
	return head
}

// https://leetcode.com/problems/number-of-good-binary-strings/description/
func goodBinaryStringsNaive(minLength int, maxLength int, oneGroup int, zeroGroup int) int {
	const mod = 1000000007

	dp := make([][]int, maxLength+1)
	for i := 0; i <= maxLength; i++ {
		dp[i] = make([]int, 2)
	}
	dp[0][0] = 1
	dp[0][1] = 1
	ans := 0

	for i := 1; i <= maxLength; i++ {
		for j := oneGroup; j <= i; j += oneGroup {
			dp[i][1] = (dp[i][1] + dp[i-j][0]) % mod
		}
		if i >= minLength {
			ans = (ans + dp[i][1]) % mod
		}
		for j := zeroGroup; j <= i; j += zeroGroup {
			dp[i][0] = (dp[i][0] + dp[i-j][1]) % mod
		}

		if i >= minLength {
			ans = (ans + dp[i][0]) % mod
		}
	}

	return ans
}

func goodBinaryStrings(minLength int, maxLength int, oneGroup int, zeroGroup int) int {
	const mod = 1000000007

	dp := make([][]int, maxLength+1)
	prefix := make([][]int, maxLength+1)
	for i := 0; i <= maxLength; i++ {
		dp[i] = make([]int, 2)
		prefix[i] = make([]int, 2)
	}

	dp[0][0] = 1
	dp[0][1] = 1
	prefix[0][0] = 1
	prefix[0][1] = 1

	ans := 0
	for i := 1; i <= maxLength; i++ {
		if i >= oneGroup {
			dp[i][1] = (dp[i][1] + prefix[i-oneGroup][0]) % mod
		}
		if i >= zeroGroup {
			dp[i][0] = (dp[i][0] + prefix[i-zeroGroup][1]) % mod
		}

		if i >= oneGroup {
			prefix[i][0] = (prefix[i-oneGroup][0] + dp[i][0]) % mod
		} else {
			prefix[i][0] = dp[i][0]
		}

		if i >= zeroGroup {
			prefix[i][1] = (prefix[i-zeroGroup][1] + dp[i][1]) % mod
		} else {
			prefix[i][1] = dp[i][1]
		}
		if i >= minLength {
			ans = (ans + dp[i][1]) % mod
			ans = (ans + dp[i][0]) % mod
		}
	}
	return ans
}

// https://leetcode.com/problems/palindrome-linked-list/description/
func isPalindrome(head *ListNode) bool {
	reverse := func(h *ListNode) *ListNode {
		var prev *ListNode
		for h != nil {
			next := h.Next
			h.Next = prev
			prev = h
			h = next
		}
		return prev
	}

	preMid := func(h *ListNode) *ListNode {
		slow, fast := h, h
		for fast.Next != nil && fast.Next.Next != nil {
			slow = slow.Next
			fast = fast.Next.Next
		}
		return slow
	}

	pre := preMid(head)
	rev := reverse(pre.Next)

	for rev != nil {
		if head.Val != rev.Val {
			return false
		}
		head = head.Next
		rev = rev.Next
	}

	return true
}

// https://leetcode.com/problems/special-permutations/description/
func specialPerm(nums []int) int {
	const mod = 1000000007

	memo := make([][]int, 1<<14+1)
	for i := 0; i < len(memo); i++ {
		memo[i] = make([]int, 16)
		for j := 0; j < len(memo[i]); j++ {
			memo[i][j] = -1
		}
	}
	var permute func(int, int, int) int
	permute = func(lastIdx int, mask int, cnt int) int {
		if cnt == len(nums) {
			return 1
		}

		if memo[mask][lastIdx+1] != -1 {
			return memo[mask][lastIdx+1]
		}

		ans := 0
		for i := 0; i < len(nums); i++ {
			if lastIdx == -1 || (mask&(1<<i) == 0 &&
				i != lastIdx &&
				(nums[lastIdx]%nums[i] == 0 || nums[i]%nums[lastIdx] == 0)) {
				ans = (ans + permute(i, mask|(1<<i), cnt+1)) % mod
			}
		}
		memo[mask][lastIdx+1] = ans
		return ans
	}

	return permute(-1, 0, 0)
}

// https://leetcode.com/problems/greatest-common-divisor-of-strings/description/
func gcdOfStrings(str1 string, str2 string) string {
	panic("todo")
}
