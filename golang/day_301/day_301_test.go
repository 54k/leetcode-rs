package day301

// https://leetcode.com/problems/combination-sum-iv/description/
func combinationSum4(nums []int, target int) int {
	memo := map[int]int{}
	var combs func(int) int
	combs = func(remain int) int {
		if remain == 0 {
			return 1
		}
		if _, ok := memo[remain]; ok {
			return memo[remain]
		}
		result := 0
		for _, num := range nums {
			if remain-num >= 0 {
				result += combs(remain - num)
			}
		}
		memo[remain] = result
		return result
	}
	return combs(target)
}
