package day275

import random "math/rand"

// https://leetcode.com/problems/kth-largest-element-in-an-array/description/
func findKthLargest(nums []int, k int) int {
	var quickselect func([]int, int) int
	quickselect = func(nums []int, k int) int {
		pivotIdx := random.Int() % len(nums)
		pivot := nums[pivotIdx]

		left := []int{}
		mid := []int{}
		right := []int{}

		for i := 0; i < len(nums); i++ {
			if nums[i] < pivot {
				right = append(right, nums[i])
			} else if nums[i] == pivot {
				mid = append(mid, nums[i])
			} else {
				left = append(left, nums[i])
			}
		}

		if len(left) >= k {
			return quickselect(left, k)
		} else if len(left)+len(mid) < k {
			return quickselect(right, k-len(left)-len(mid))
		}
		return pivot
	}
	return quickselect(nums, k)
}

// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/description/
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func sortedArrayToBST(nums []int) *TreeNode {
	var dfs func(int, int) *TreeNode
	dfs = func(lo int, hi int) *TreeNode {
		if lo > hi {
			return nil
		}

		m := (lo + hi) / 2
		mid := &TreeNode{nums[m], nil, nil}
		mid.Left = dfs(lo, m-1)
		mid.Right = dfs(m+1, hi)
		return mid
	}

	return dfs(0, len(nums)-1)
}

// https://leetcode.com/problems/maximum-gap/description/
func maximumGap(nums []int) int {
	type Bucket struct {
		used   bool
		minVal int
		maxVal int
	}

	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	if len(nums) == 0 || len(nums) < 2 {
		return 0
	}
	mini, maxi := (1 << 30), -(1 << 30)
	for _, v := range nums {
		if v < mini {
			mini = v
		}
		if v > maxi {
			maxi = v
		}
	}

	bucketSize := max(1, (maxi-mini)/len(nums)-1) // bucket size or capacity
	bucketNum := (maxi-mini)/bucketSize + 1       // number of buckets
	buckets := make([]*Bucket, bucketNum)
	for i := 0; i < len(buckets); i++ {
		buckets[i] = &Bucket{false, (1 << 30), -(1 << 30)}
	}

	for _, num := range nums {
		bucketIdx := (num - mini) / bucketSize
		buckets[bucketIdx].used = true
		buckets[bucketIdx].minVal = min(num, buckets[bucketIdx].minVal)
		buckets[bucketIdx].maxVal = max(num, buckets[bucketIdx].maxVal)
	}

	prevBucketMax := mini
	maxGap := 0

	for _, bucket := range buckets {
		if !bucket.used {
			continue
		}
		maxGap = max(maxGap, bucket.minVal-prevBucketMax)
		prevBucketMax = bucket.maxVal
	}
	return maxGap
}
