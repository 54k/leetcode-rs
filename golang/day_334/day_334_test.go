package day334

type MountainArray struct {
}

func (this *MountainArray) get(index int) int { return 0 }
func (this *MountainArray) length() int       { return 0 }

// https://leetcode.com/problems/find-in-mountain-array/description
func findInMountainArray(target int, mountainArr *MountainArray) int {
	length := mountainArr.length()
	cache := map[int]int{}

	lo := 1
	hi := length - 2

	for lo != hi {
		testIndex := (lo + hi) >> 1
		var curr int
		if val, ok := cache[testIndex]; ok {
			curr = val
		} else {
			curr = mountainArr.get(testIndex)
			cache[testIndex] = curr
		}

		var next int
		if val, ok := cache[testIndex+1]; ok {
			next = val
		} else {
			next = mountainArr.get(testIndex + 1)
			cache[testIndex+1] = next
		}

		if curr < next {
			if curr == target {
				return testIndex
			}
			if next == target {
				return testIndex + 1
			}
			lo = testIndex + 1
		} else {
			hi = testIndex
		}
	}
	peakIndex := lo

	lo = 0
	hi = peakIndex
	for lo <= hi {
		testIndex := (lo + hi) >> 1
		var curr int
		if val, ok := cache[testIndex]; ok {
			curr = val
		} else {
			curr = mountainArr.get(testIndex)
			cache[testIndex] = curr
		}
		if curr == target {
			return testIndex
		} else if curr < target {
			lo = testIndex + 1
		} else {
			hi = testIndex - 1
		}
	}

	lo = peakIndex + 1
	hi = length - 1
	for lo <= hi {
		testIndex := (lo + hi) >> 1
		var curr int
		if val, ok := cache[testIndex]; ok {
			curr = val
		} else {
			curr = mountainArr.get(testIndex)
			cache[testIndex] = curr
		}
		if curr == target {
			return testIndex
		} else if curr < target {
			hi = testIndex - 1
		} else {
			lo = testIndex + 1
		}
	}
	return -1
}

// https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/description/
func minimumMountainRemovals(nums []int) int {
	lisUp := make([]int, len(nums))
	for i := 0; i < len(nums); i++ {
		lisUp[i] = 1
		for j := 0; j < i; j++ {
			if nums[i] > nums[j] {
				if lisUp[i] < lisUp[j]+1 {
					lisUp[i] = lisUp[j] + 1
				}
			}
		}
	}

	lisDown := make([]int, len(nums))
	for i := len(nums) - 1; i >= 0; i-- {
		lisDown[i] = 1
		for j := len(nums) - 1; j > i; j-- {
			if nums[i] > nums[j] {
				if lisDown[i] < lisDown[j]+1 {
					lisDown[i] = lisDown[j] + 1
				}
			}
		}
	}

	ans := 0
	for i := 0; i < len(nums); i++ {
		if lisUp[i] >= 2 && lisDown[i] >= 2 {
			if ans < lisUp[i]+lisDown[i]-1 {
				ans = lisUp[i] + lisDown[i] - 1
			}
		}
	}
	return len(nums) - ans
}
