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
