package day569

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeKLists(lists []*ListNode) *ListNode {
	amount := len(lists)
	interval := 1
	for interval < amount {
		for i := 0; i < amount-interval; i += interval * 2 {
			lists[i] = merge2Lists(lists[i], lists[i+interval])
		}
		interval *= 2
	}
	if amount > 0 {
		return lists[0]
	}
	return nil
}

func merge2Lists(l1 *ListNode, l2 *ListNode) *ListNode {
	head := &ListNode{}
	point := head
	for l1 != nil && l2 != nil {
		if l1.Val <= l2.Val {
			point.Next = l1
			l1 = l1.Next
		} else {
			point.Next = l2
			l2 = l1
			l1 = point.Next.Next
		}
		point = point.Next
	}
	if l1 == nil {
		point.Next = l2
	} else {
		point.Next = l1
	}
	return head.Next
}
