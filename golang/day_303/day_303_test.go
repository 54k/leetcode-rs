package day303

// https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/submissions
func groupThePeople(groupSizes []int) [][]int {
	ans := [][]int{}
	groups := map[int][]int{}
	for i := 0; i < len(groupSizes); i++ {
		if _, ok := groups[groupSizes[i]]; !ok {
			groups[groupSizes[i]] = []int{}
		}
		groups[groupSizes[i]] = append(groups[groupSizes[i]], i)

		if len(groups[groupSizes[i]]) == groupSizes[i] {
			ans = append(ans, groups[groupSizes[i]])
			delete(groups, groupSizes[i])
		}
	}
	return ans
}

// https://leetcode.com/problems/split-array-largest-sum/description/
func splitArray(nums []int, k int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	memo := map[int]map[int]int{}
	for i := 0; i < 1001; i++ {
		memo[i] = map[int]int{}
	}

	n := len(nums)
	prefixSum := make([]int, n+1)
	for i := 0; i < n; i++ {
		prefixSum[i+1] = prefixSum[i] + nums[i]
	}

	var getMinimumLargestSplitSum func(int, int) int
	getMinimumLargestSplitSum = func(currIndex int, subarrayCount int) int {
		n := len(prefixSum) - 1

		if _, ok := memo[currIndex][subarrayCount]; ok {
			return memo[currIndex][subarrayCount]
		}

		if subarrayCount == 1 {
			memo[currIndex][subarrayCount] = prefixSum[n] - prefixSum[currIndex]
			return memo[currIndex][subarrayCount]
		}

		mininumLargestSplitSum := (1 << 31)
		for i := 0; i <= n-subarrayCount; i++ {
			firstSplitSum := prefixSum[i+1] - prefixSum[currIndex]

			largestSplitSum := max(firstSplitSum, getMinimumLargestSplitSum(i+1, subarrayCount-1))

			mininumLargestSplitSum = min(mininumLargestSplitSum, largestSplitSum)
			if firstSplitSum >= mininumLargestSplitSum {
				break
			}
		}

		memo[currIndex][subarrayCount] = mininumLargestSplitSum
		return memo[currIndex][subarrayCount]
	}

	return getMinimumLargestSplitSum(0, k)
}

func splitArrayBinarySearch(nums []int, k int) int {
	canSplit := func(mid int) bool {
		ans := 0
		sum := 0
		for _, n := range nums {
			if sum+n <= mid {
				sum += n
			} else {
				sum = n
				ans++
			}
		}
		return ans+1 <= k
	}

	left, right := 0, 0
	for _, n := range nums {
		if n > left {
			left = n
		}
		right += n
	}

	for left < right {
		mid := left + (right-left)/2
		if canSplit(mid) {
			right = mid
		} else {
			left = mid + 1
		}
	}
	return left
}
