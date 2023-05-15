package main

// https://leetcode.com/problems/swapping-nodes-in-a-linked-list/description/
func swapNodes(head *ListNode, k int) *ListNode {
	len := 0
	var fronNode *ListNode = nil
	var endNode *ListNode = nil
	currentNode := head

	for currentNode != nil {
		len++

		if endNode != nil {
			endNode = endNode.Next
		}

		if len == k {
			fronNode = currentNode
			endNode = head
		}

		currentNode = currentNode.Next
	}

	temp := fronNode.Val
	fronNode.Val = endNode.Val
	endNode.Val = temp
	return head
}

func sortArray(nums []int) []int {
	merge := func(nums, aux []int, left, mid, right int) {
		k, i, j := left, left, mid+1
		for i <= mid && j <= right {
			if nums[i] <= nums[j] {
				aux[k] = nums[i]
				i++
			} else if nums[j] < nums[i] {
				aux[k] = nums[j]
				j++
			}
			k++
		}

		for ; i <= mid; i++ {
			aux[k] = nums[i]
			k++
		}

		for ; j <= right; j++ {
			aux[k] = nums[j]
			k++
		}

		for ; left <= right; left++ {
			nums[left] = aux[left]
		}
	}
	var mergeSort func(nums, dst []int, left, right int)
	mergeSort = func(nums, dst []int, left, right int) {
		if left >= right {
			return
		}

		mid := (left + right) / 2
		mergeSort(nums, dst, left, mid)
		mergeSort(nums, dst, mid+1, right)

		merge(nums, dst, left, mid, right)
	}

	mergeSort(nums, make([]int, len(nums)), 0, len(nums)-1)
	return nums
}

func isValidBST(root *TreeNode) bool {
	approach1 := func(root *TreeNode) bool {
		var check func(root *TreeNode, lo, hi *int) bool
		check = func(root *TreeNode, lo, hi *int) bool {
			if root == nil {
				return true
			}

			if lo != nil && root.Val <= *lo {
				return false
			}
			if hi != nil && root.Val >= *hi {
				return false
			}

			return check(root.Left, lo, &root.Val) && check(root.Right, &root.Val, hi)
		}
		return check(root, nil, nil)
	}

	approach2 := func(root *TreeNode) bool {
		var inorder func(root *TreeNode, acc *[]int) bool
		inorder = func(root *TreeNode, acc *[]int) bool {
			if root != nil {
				if !inorder(root.Left, acc) {
					return false
				}

				if len(*acc) > 0 && (*acc)[len(*acc)-1] >= root.Val {
					return false
				}
				*acc = append(*acc, root.Val)
				return inorder(root.Right, acc)
			}
			return true
		}
		return inorder(root, &[]int{})
	}
	return approach1(root) == approach2(root)
}

func inorderSuccessor(root *TreeNode, p *TreeNode) *TreeNode {
	var succ *TreeNode

	for root != nil {
		if p.Val >= root.Val {
			root = root.Right
		} else {
			succ = root
			root = root.Left
		}
	}

	return succ
}
