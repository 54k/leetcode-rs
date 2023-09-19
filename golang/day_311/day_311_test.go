package day311

// https://leetcode.com/problems/find-the-duplicate-number/description
func findDuplicateNegativeMarking(nums []int) int {
	abs := func(a int) int {
		if a < 0 {
			return -a
		}
		return a
	}
	duplicate := -1
	for i := 0; i < len(nums); i++ {
		cur := abs(nums[i])
		if nums[cur] < 0 {
			duplicate = cur
			break
		}
		nums[cur] *= -1
	}
	for i := 0; i < len(nums); i++ {
		nums[i] = abs(nums[i])
	}
	return duplicate
}

func findDuplicateArrayAsHashMapRecursive(nums []int) int {
	var store func(int) int
	store = func(cur int) int {
		if nums[cur] == cur {
			return cur
		}
		next := nums[cur]
		nums[cur] = cur
		return store(next)
	}
	return store(0)
}

func findDuplicateArrayAsHashMapIterative(nums []int) int {
	for nums[0] != nums[nums[0]] {
		nums[0], nums[nums[0]] = nums[nums[0]], nums[0]
	}
	return nums[0]
}

func findDuplicateFloydAlgorithm(nums []int) int {
	slow, fast := nums[0], nums[0]
	for {
		slow = nums[slow]
		fast = nums[nums[fast]]
		if slow == fast {
			slow = nums[0]
			for slow != fast {
				slow = nums[slow]
				fast = nums[fast]
			}
			return slow
		}
	}
}
