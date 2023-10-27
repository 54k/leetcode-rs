package day349

// https://leetcode.com/problems/burst-balloons/description/
func maxCoinsTopDown(nums []int) int {
	arr := make([]int, len(nums)+2)
	for i := 0; i < len(arr); i++ {
		arr[i] = 1
	}
	for i := 0; i < len(nums); i++ {
		arr[i+1] = nums[i]
	}

	nums = arr
	n := len(nums)

	memo := make([][]int, n)
	for i := 0; i < n; i++ {
		memo[i] = make([]int, n)
	}

	var dp func(int, int) int
	dp = func(left, right int) int {
		if right-left < 0 {
			return 0
		}
		if memo[left][right] > 0 {
			return memo[left][right]
		}
		ans := 0
		for i := left; i <= right; i++ {
			rest := dp(left, i-1) + dp(i+1, right)
			gain := nums[i] * nums[left-1] * nums[right+1]
			if ans < rest+gain {
				ans = rest + gain
			}
		}
		memo[left][right] = ans
		return ans
	}

	return dp(1, n-2)
}

func maxCoinsBottomUp1(nums []int) int {
	arr := make([]int, len(nums)+2)
	for i := 0; i < len(arr); i++ {
		arr[i] = 1
	}
	for i := 0; i < len(nums); i++ {
		arr[i+1] = nums[i]
	}

	nums = arr
	n := len(nums)

	dp := make([][]int, n)
	for i := 0; i < n; i++ {
		dp[i] = make([]int, n)
	}

	for diff := 1; diff < n; diff++ {
		for left := 1; left < n-diff; left++ {
			right := left + diff - 1

			ans := 0
			for k := left; k <= right; k++ {

				rest := dp[left][k-1] + dp[k+1][right]
				gain := nums[k] * nums[left-1] * nums[right+1]

				if ans < rest+gain {
					ans = rest + gain
				}
			}

			if dp[left][right] < ans {
				dp[left][right] = ans
			}
		}
	}

	return dp[1][n-2]
}

func maxCoinsBottomUp2(nums []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	arr := make([]int, len(nums)+2)
	for i := 0; i < len(arr); i++ {
		arr[i] = 1
	}
	for i := 0; i < len(nums); i++ {
		arr[i+1] = nums[i]
	}

	nums = arr
	n := len(nums)

	dp := make([][]int, n)
	for i := 0; i < n; i++ {
		dp[i] = make([]int, n)
	}

	for left := n - 2; left >= 1; left-- {
		for right := left; right <= n-2; right++ {

			for i := left; i <= right; i++ {
				gain := nums[left-1] * nums[i] * nums[right+1]
				remaining := dp[left][i-1] + dp[i+1][right]

				dp[left][right] = max(remaining+gain, dp[left][right])
			}
		}
	}

	return dp[1][n-2]
}
