package day_250

// https://leetcode.com/problems/asteroid-collision/description/
func asteroidCollision(asteroids []int) []int {
	abs := func(i int) int {
		if i < 0 {
			return -i
		}
		return i
	}
	stack := []int{}
label:
	for _, a := range asteroids {
		for len(stack) > 0 && stack[len(stack)-1] > 0 && a < 0 {
			absA, absB := abs(a), abs(stack[len(stack)-1])
			if absA < absB {
				continue label
			}
			stack = stack[:len(stack)-1]
			if absA == absB {
				continue label
			}
		}
		stack = append(stack, a)
	}
	return stack
}

// https://leetcode.com/problems/palindrome-linked-list/description/
type ListNode struct {
	Val  int
	Next *ListNode
}

func isPalindromeRec(head *ListNode) bool {
	frontPointer := head
	var checkRec func(*ListNode) bool
	checkRec = func(h *ListNode) bool {
		if h != nil {
			if !checkRec(h.Next) {
				return false
			}
			if h.Val != frontPointer.Val {
				return false
			}
			frontPointer = frontPointer.Next
		}
		return true
	}

	return checkRec(head)
}

func isPalindromeRevNoMem(head *ListNode) bool {
	mid := func(h *ListNode) *ListNode {
		slow, fast := h, h
		for fast.Next != nil && fast.Next.Next != nil {
			fast = fast.Next.Next
			slow = slow.Next
		}
		return slow
	}

	rev := func(h *ListNode) *ListNode {
		var prev *ListNode
		for h != nil {
			next := h.Next
			h.Next = prev
			prev = h
			h = next
		}
		return prev
	}

	eq := func(a, b *ListNode) bool {
		result := true
		for result && b != nil {
			if a.Val != b.Val {
				result = false
			}
			a = a.Next
			b = b.Next
		}
		return result
	}

	firstHalfEnd := mid(head)
	secHalfStart := rev(firstHalfEnd.Next)

	res := eq(head, secHalfStart)
	rev(firstHalfEnd.Next)
	return res
}

// https://leetcode.com/problems/middle-of-the-linked-list/description/
func middleNode(head *ListNode) *ListNode {
	slow, fast := head, head
	for fast != nil && fast.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next
	}
	return slow
}
