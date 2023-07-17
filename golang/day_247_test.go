package main

// https://leetcode.com/problems/add-two-numbers-ii/description/
func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	reverse := func(list *ListNode) *ListNode {
		var prev *ListNode
		for list != nil {
			tmp := list.Next
			list.Next = prev
			prev = list
			list = tmp
		}
		return prev
	}
	l1 = reverse(l1)
	l2 = reverse(l2)
	total := 0
	var ans *ListNode
	for l1 != nil || l2 != nil {
		if l1 != nil {
			total += l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			total += l2.Val
			l2 = l2.Next
		}
		ans = &ListNode{total % 10, ans}
		total /= 10
	}
	if total > 0 {
		ans = &ListNode{total, ans}
	}
	return ans
}
