package day868

// https://leetcode.com/problems/sum-of-all-subset-xor-totals/description/?envType=daily-question&envId=2025-04-05
func subsetXORSum(nums []int) int {
    var xorSum func(nums []int, index int, currentXor int) int
    xorSum = func(nums []int, index, currentXor int) int {
        if index == len(nums) {
            return currentXor
        }

        withElement := xorSum(nums, index + 1, currentXor ^ nums[index])
        withoutElement := xorSum(nums, index + 1, currentXor)

        return withElement + withoutElement
    }

    return xorSum(nums, 0, 0)
}