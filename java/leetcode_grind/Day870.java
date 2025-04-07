package leetcode_grind;

public class Day870 {
    // https://leetcode.com/problems/partition-equal-subset-sum/description/?envType=daily-question&envId=2025-04-07
    static class Solution1 {
        public boolean canPartition(int[] nums) {
            int totalSum = 0;
            for (int num : nums)
                totalSum += num;
            if (totalSum % 2 == 1)
                return false;
            int targetSum = totalSum / 2;
            boolean[] dp = new boolean[targetSum + 1];
            dp[0] = true;

            for (int num : nums) {
                for (int currSum = targetSum; currSum >= num; currSum--) {
                    dp[currSum] |= dp[currSum - num];
                    if (dp[targetSum])
                        return true;
                }
            }

            return dp[targetSum];
        }
    }
}
