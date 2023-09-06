package day298

import (
	"fmt"
	"testing"
)

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

// https://leetcode.com/problems/sort-an-array/description/
func sortArrayCountingSort(nums []int) []int {
	abs := func(a int) int {
		if a < 0 {
			return -a
		}
		return a
	}
	bucketSort := func(nums []int, place int) {
		buckets := make([][]int, 10)
		for i := 0; i < len(buckets); i++ {
			buckets[i] = []int{}
		}

		for _, n := range nums {
			digit := abs(n) / place
			digit %= 10
			buckets[digit] = append(buckets[digit], n)
		}

		i := 0
		for _, b := range buckets {
			for _, n := range b {
				nums[i] = n
				i++
			}
		}
	}

	maxi := nums[0]
	for _, n := range nums {
		if maxi < n {
			maxi = n
		}
	}

	totalDigits := 0
	for ; maxi > 0; totalDigits++ {
		maxi /= 10
	}

	for place := 1; totalDigits >= 0; place *= 10 {
		bucketSort(nums, place)
		totalDigits--
	}

	negs := []int{}
	posi := []int{}
	for _, n := range nums {
		if n < 0 {
			negs = append(negs, n)
		} else {
			posi = append(posi, n)
		}
	}

	for i := 0; i < len(negs)/2; i++ {
		negs[i], negs[len(negs)-1-i] = negs[len(negs)-1-i], negs[i]
	}

	i := 0
	for _, n := range negs {
		nums[i] = n
		i++
	}

	for _, n := range posi {
		nums[i] = n
		i++
	}

	return nums
}

func TestSort(t *testing.T) {
	fmt.Println(sortArrayCountingSort([]int{-1, 2, -8, -10}))
}
