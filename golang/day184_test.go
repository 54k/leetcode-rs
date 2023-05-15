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

func isValid(root *TreeNode, lo, hi *int) bool {
	if root == nil {
		return true
	}
	v := root.Val
	if lo != nil && v <= *lo {
		return false
	}
	if hi != nil && v >= *hi {
		return false
	}
	return isValid(root.Left, lo, &v) && isValid(root.Right, &v, hi)
}

func isValidIt(root *TreeNode) bool {
	stack := []*TreeNode{}
	mins, maxs := []*int{}, []*int{}

	update := func(v *TreeNode, lo, hi *int) {
		stack = append(stack, v)
		mins = append(mins, lo)
		maxs = append(maxs, hi)
	}

	var min *int = nil
	var max *int = nil
	update(root, min, max)

	for len(stack) > 0 {
		el := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		min = mins[len(mins)-1]
		mins = mins[:len(mins)-1]

		max = maxs[len(maxs)-1]
		maxs = maxs[:len(maxs)-1]

		if el == nil {
			continue
		}

		v := el.Val
		if min != nil && v <= *min {
			return false
		}
		if max != nil && v >= *max {
			return false
		}

		update(el.Right, &v, max)
		update(el.Left, min, &v)
	}
	return true
}

func isValidInorder(root *TreeNode) bool {
	var prev *int
	var inorder func(*TreeNode) bool
	inorder = func(root *TreeNode) bool {
		if root == nil {
			return true
		}
		if !inorder(root.Left) {
			return false
		}
		if prev != nil && *prev >= root.Val {
			return false
		}
		prev = &root.Val
		return inorder(root.Right)
	}
	return inorder(root)
}

func isValidInorderIt(root *TreeNode) bool {
	var prev *int
	stack := []*TreeNode{}

	for len(stack) > 0 || root != nil {
		for root != nil {
			stack = append(stack, root)
			root = root.Left
		}
		root = stack[len(stack)-1]
		stack = stack[0 : len(stack)-1]

		if prev != nil && root.Val <= *prev {
			return false
		}

		prev = &root.Val
		root = root.Right
	}
	return true
}

func isValidBST(root *TreeNode) bool {
	return isValidInorderIt(root)
}
