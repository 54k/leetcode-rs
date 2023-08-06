package day267

// https://leetcode.com/problems/add-two-numbers/description/
type ListNode struct {
	Val  int
	Next *ListNode
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	ans := &ListNode{0, nil}
	next := ans
	sum := 0
	for l1 != nil || l2 != nil {
		if l1 != nil {
			sum += l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			sum += l2.Val
			l2 = l2.Next
		}
		node := &ListNode{sum % 10, nil}
		next.Next = node
		next = next.Next
		sum /= 10
	}
	if sum > 0 {
		node := &ListNode{sum % 10, nil}
		next.Next = node
	}
	return ans.Next
}

// https://leetcode.com/problems/merge-two-sorted-lists/description/
func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	if list1 == nil {
		return list2
	}
	if list2 == nil {
		return list1
	}
	if list1.Val <= list2.Val {
		list1.Next = mergeTwoLists(list1.Next, list2)
		return list1
	}
	list2.Next = mergeTwoLists(list1, list2.Next)
	return list2
}

// https://leetcode.com/problems/greatest-common-divisor-of-strings/description/
func gcdOfStrings(str1 string, str2 string) string {
	panic("todo")
}
