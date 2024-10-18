package leetcode_grind;

public class Day702 {
    // https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/description/?envType=daily-question&envId=2024-10-18
    static class Solution1 {
        public int countMaxOrSubsets(int[] nums) {
            int max = 0;
            int[] dp = new int[1 << 17];
            dp[0] = 1;
            for (int num : nums) {
                for (int i = max; i >= 0; i--) {
                    dp[i | num] += dp[i];
                }
                max |= num;
            }
            return dp[max];
        }
    }
}
