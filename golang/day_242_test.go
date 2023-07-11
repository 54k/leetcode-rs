package main

import "sort"

func distributeCookies(cookies []int, k int) int {
	distribute := make([]int, k)
	const MAX = 2_147_483_647
	const MIN = -2_147_483_648

	var dfs func(int, int) int
	dfs = func(i int, zero_count int) int {
		if len(cookies)-i < zero_count {
			return MAX
		}

		if i == len(cookies) {
			unfairness := MIN
			for _, val := range distribute {
				if unfairness < val {
					unfairness = val
				}
			}
			return unfairness
		}

		answer := MAX
		for j := 0; j < k; j++ {
			if distribute[j] == 0 {
				zero_count -= 1
			}
			distribute[j] += cookies[i]
			d := dfs(i+1, zero_count)
			if answer > d {
				answer = d
			}
			distribute[j] -= cookies[i]
			if distribute[j] == 0 {
				zero_count += 1
			}
		}

		return answer
	}

	return dfs(0, k)
}

func threeSum(nums []int) [][]int {
	n := len(nums)
	ans := [][]int{}
	sort.Ints(nums)

	var twoSum func(int)
	twoSum = func(i int) {
		left, right := i+1, n-1
		for left < right {
			sum := nums[i] + nums[left] + nums[right]
			if sum < 0 {
				left++
			} else if sum > 0 {
				right--
			} else {
				ans = append(ans, []int{nums[left], nums[right], nums[i]})
				left++
				right--
				for left < right && nums[left] == nums[left-1] {
					left++
				}
			}
		}
	}

	for i := 0; i < n && nums[i] <= 0; i++ {
		if i == 0 || nums[i-1] != nums[i] {
			twoSum(i)
		}
	}

	return ans
}

type NodeCon struct {
	Val   int
	Left  *NodeCon
	Right *NodeCon
	Next  *NodeCon
}

func connect_ONmem(root *NodeCon) *NodeCon {
	if root == nil {
		return root
	}

	q := []*NodeCon{root}

	for len(q) > 0 {
		size := len(q)

		for i := 0; i < size; i++ {
			node := q[0]
			q = q[1:]

			if i < size-1 {
				node.Next = q[0]
			}

			if node.Left != nil {
				q = append(q, node.Left)
			}

			if node.Right != nil {
				q = append(q, node.Right)
			}
		}
	}

	return root
}

func connect_O1mem(root *Node) *Node {
	if root == nil {
		return root
	}

	// Start with the root node. There are no next pointers
	// that need to be set up on the first level
	leftmost := root

	// Once we reach the final level, we are done
	for leftmost.Left != nil {
		// Iterate the "linked list" starting from the head
		// node and using the next pointers, establish the
		// corresponding links for the next level
		head := leftmost

		for head != nil {
			// connection 1
			head.Left.Next = head.Right

			// connection 2
			if head.Next != nil {
				head.Right.Next = head.Next.Left
			}

			head = head.Next
		}

		leftmost = leftmost.Left
	}

	return root
}
