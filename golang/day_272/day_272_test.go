package day272

// https://leetcode.com/problems/coin-change-ii/description/
func change(amount int, coins []int) int {
	dp := make([]int, amount+1)
	dp[0] = 1
	for _, coin := range coins {
		for amt := 1; amt <= amount; amt++ {
			if amt >= coin {
				dp[amt] += dp[amt-coin]
			}
		}
	}
	return dp[amount]
}

// https://leetcode.com/problems/insertion-sort-list/
type ListNode struct {
	Val  int
	Next *ListNode
}

func insertionSortList(head *ListNode) *ListNode {
	dummy := &ListNode{0, nil}
	curr := head
	for curr != nil {
		prev := dummy

		for prev.Next != nil && prev.Next.Val <= curr.Val {
			prev = prev.Next
		}

		next := curr.Next
		curr.Next = prev.Next
		prev.Next = curr
		curr = next
	}
	return dummy.Next
}
