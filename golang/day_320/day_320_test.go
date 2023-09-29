package day320

import "strconv"

// https://leetcode.com/problems/sort-array-by-parity/description
func sortArrayByParityLomuto(nums []int) []int {
	left := 0
	for right := 0; right < len(nums); right++ {
		if nums[right]&1 == 0 {
			nums[left], nums[right] = nums[right], nums[left]
			left++
		}
	}
	return nums
}

func sortArrayByParityHoare(nums []int) []int {
	left, right := 0, len(nums)-1
	for left != right {
		for left != right && nums[left]&1 == 0 {
			left++
		}
		for left != right && nums[right]&1 == 1 {
			right--
		}
		nums[left], nums[right] = nums[right], nums[left]
	}
	return nums
}

// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/
func findMaximumXORPrefixSet(nums []int) int {
	L, max := 0, nums[0]
	for i := 1; i < len(nums); i++ {
		if nums[i] > max {
			max = nums[i]
		}
	}
	for max > 0 {
		L++
		max /= 2
	}

	maxXor := 0
	for i := L - 1; i >= 0; i-- {
		maxXor <<= 1
		curXor := maxXor | 1

		prefixes := map[int]bool{}
		for _, n := range nums {
			prefixes[n>>i] = true
		}

		for n, _ := range prefixes {
			xor := curXor ^ n
			if prefixes[xor] {
				maxXor = curXor
				break
			}
		}
	}
	return maxXor
}

func findMaximumXORTrie(nums []int) int {
	type trieNode struct {
		children map[rune]*trieNode
	}

	L, max := 0, nums[0]
	for i := 1; i < len(nums); i++ {
		if nums[i] > max {
			max = nums[i]
		}
	}
	for max > 0 {
		L++
		max /= 2
	}

	trie := &trieNode{map[rune]*trieNode{}}
	strNums := []string{}
	for _, n := range nums {
		strNums = append(strNums, strconv.FormatInt(int64(n|(1<<L)), 2)[1:])
	}

	maxXor := 0
	for _, n := range strNums {
		node, xorNode := trie, trie
		currXor := 0
		for _, bit := range n {
			opposite := '0'
			if bit == '0' {
				opposite = '1'
			}

			if _, ok := node.children[bit]; !ok {
				node.children[bit] = &trieNode{map[rune]*trieNode{}}
			}
			node = node.children[bit]

			if _, ok := xorNode.children[opposite]; ok {
				currXor = (currXor << 1) | 1
				xorNode = xorNode.children[opposite]
			} else {
				currXor <<= 1
				xorNode = xorNode.children[bit]
			}
		}
		if maxXor < currXor {
			maxXor = currXor
		}
	}
	return maxXor
}
