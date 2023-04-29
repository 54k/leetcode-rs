package main

import (
	"fmt"
	"testing"
)

func merge(nums1 []int, m int, nums2 []int, n int) {
	p1 := m - 1
	p2 := n - 1

	for p := m + n - 1; p >= 0; p-- {
		if p2 < 0 {
			return
		}
		if p1 >= 0 && nums1[p1] > nums2[p2] {
			nums1[p] = nums1[p1]
			p1--
		} else {
			nums1[p] = nums2[p2]
			p2--
		}
	}
}

func removeElement(nums []int, val int) int {
	i := 0
	n := len(nums)

	for i < n {
		if nums[i] == val {
			nums[i] = nums[n-1]
			n--
		} else {
			i++
		}
	}
	return i
}

func removeDuplicates(nums []int) int {
	i := 1
	for j := 1; j < len(nums); j++ {
		if nums[j] != nums[j-1] {
			nums[i] = nums[j]
			i++
		}
	}
	return i
}

func checkIfExist(arr []int) bool {
	m := make(map[int]bool)
	for i := 0; i < len(arr); i++ {
		if m[arr[i]*2] || arr[i]%2 == 0 && m[arr[i]/2] {
			return true
		}
		m[arr[i]] = true
	}
	return false
}

func validMountainArray(arr []int) bool {
	i, n := 0, len(arr)
	for ; i+1 < n && arr[i] < arr[i+1]; i++ {
	}
	if i == 0 || i == n-1 {
		return false
	}
	for ; i+1 < n && arr[i] > arr[i+1]; i++ {
	}

	return i == n-1
}

func TestMerge(t *testing.T) {
	nums := []int{1, 2, 3, 0, 0, 0}
	merge(nums, 3, []int{1, 2, 3}, 3)
	fmt.Printf("nums: %v\n", nums) // 1,1,2,2,3,3
}

func TestRemoveElement(t *testing.T) {
	fmt.Printf("removeElement([]int{3, 2, 2, 3}, 3): %v\n", removeElement([]int{3, 2, 2, 3}, 3)) // 2
}

func TestRemoveDuplicates(t *testing.T) {
	fmt.Printf("removeDuplicates([]int{0, 0, 1, 1, 1, 2, 2, 3, 3, 4}): %v\n", removeDuplicates([]int{0, 0, 1, 1, 1, 2, 2, 3, 3, 4})) // 5
}

func TestCheckIfExist(t *testing.T) {
	fmt.Printf("checkIfExist([]int{10, 2, 5, 3}): %v\n", checkIfExist([]int{10, 2, 5, 3})) // true
}

func TestValidMountainArray(t *testing.T) {
	fmt.Printf("validMountainArray([]int{0, 3, 2, 1}): %v\n", validMountainArray([]int{0, 3, 2, 1})) // true
	fmt.Printf("validMountainArray([]int{3, 5, 5}): %v\n", validMountainArray([]int{3, 5, 5}))       // false
}
