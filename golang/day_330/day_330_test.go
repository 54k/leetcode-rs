package day330

// https://leetcode.com/problems/max-dot-product-of-two-subsequences/description
func maxDotProduct(nums1 []int, nums2 []int) int {
	max := func(a, b int) int {
		if a < b {
			return b
		}
		return a
	}
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	firstMax := -(1 << 31)
	firstMin := 1 << 31
	secondMax := -(1 << 31)
	secondMin := 1 << 31

	for _, n := range nums1 {
		firstMax = max(firstMax, n)
		firstMin = min(firstMin, n)
	}

	for _, n := range nums2 {
		secondMax = max(secondMax, n)
		secondMin = min(secondMin, n)
	}

	memo := make([][]int, len(nums1))
	for i := 0; i < len(memo); i++ {
		memo[i] = make([]int, len(nums2))
	}

	if firstMax < 0 && secondMin > 0 {
		return firstMax * secondMin
	}

	if secondMax < 0 && firstMin > 0 {
		return secondMax * firstMin
	}

	var dp func(int, int) int
	dp = func(i, j int) int {
		if i == len(nums1) || j == len(nums2) {
			return 0
		}

		if memo[i][j] != 0 {
			return memo[i][j]
		}

		ans := -(1 << 31)
		a := dp(i+1, j)
		b := dp(i, j+1)
		c := nums1[i]*nums2[j] + dp(i+1, j+1)
		ans = max(max(a, b), c)
		memo[i][j] = ans
		return ans
	}

	return dp(0, 0)
}
