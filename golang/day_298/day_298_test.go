package day298

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
