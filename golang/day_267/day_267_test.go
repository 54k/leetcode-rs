package day267

import "sort"

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

// https://leetcode.com/problems/maximum-elegance-of-a-k-length-subsequence/description/
func findMaximumElegance(items [][]int, k int) int64 {
	max := func(a, b int64) int64 {
		if a > b {
			return a
		}
		return b
	}
	dups := []int{}
	sort.Slice(items, func(a, b int) bool {
		return items[a][0] > items[b][0]
	})
	cur := int64(0)
	res := int64(0)
	seen := map[int]bool{}
	for i := 0; i < len(items); i++ {
		if i < k {
			if seen[items[i][1]] {
				dups = append(dups, items[i][0])
			}
			cur += int64(items[i][0])
		} else if !seen[items[i][1]] {
			if len(dups) == 0 {
				break
			}
			cur += int64(items[i][0] - dups[len(dups)-1])
			dups = dups[:len(dups)-1]
		}
		seen[items[i][1]] = true
		res = max(res, cur+int64(len(seen))*int64(len(seen)))
	}
	return res
}

// https://leetcode.com/problems/greatest-common-divisor-of-strings/description/
func gcdOfStrings(str1 string, str2 string) string {
	panic("todo")
}
