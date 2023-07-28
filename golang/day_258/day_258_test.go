package day258

// https://leetcode.com/problems/predict-the-winner/description/
func PredictTheWinner(nums []int) bool {
	memo := make([][]int, len(nums))
	for i := 0; i < len(nums); i++ {
		memo[i] = make([]int, len(nums))
		for j := 0; j < len(nums); j++ {
			memo[i][j] = -1
		}
	}
	var dp func(int, int) int
	dp = func(left int, right int) int {
		if memo[left][right] != -1 {
			return memo[left][right]
		}
		if left == right {
			return nums[left]
		}

		leftScore := nums[left] - dp(left+1, right)
		rightScore := nums[right] - dp(left, right-1)
		ans := 0
		if leftScore > rightScore {
			ans = leftScore
		} else {
			ans = rightScore
		}
		memo[left][right] = ans
		return ans
	}

	return dp(0, len(nums)-1) >= 0
}
