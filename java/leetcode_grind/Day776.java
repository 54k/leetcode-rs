package leetcode_grind;

public class Day776 {
    // https://leetcode.com/problems/number-of-ways-to-split-array/description/?envType=daily-question&envId=2025-01-03
    static class Solution1 {
        public int waysToSplitArray(int[] nums) {
            long leftSum = 0, rightSum = 0;
            for (int num : nums) {
                rightSum += num;
            }

            int count = 0;
            for (int i = 0; i < nums.length - 1; i++) {
                leftSum += nums[i];
                rightSum -= nums[i];

                if (leftSum >= rightSum) {
                    count++;
                }
            }
            return count;
        }
    }
}
