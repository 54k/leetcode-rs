package leetcode_grind;

import java.util.Arrays;

public class Day916 {
    // https://leetcode.com/problems/find-the-maximum-sum-of-node-values/description/
    static class Solution1 {
        public long maximumValueSum(int[] nums, int k, int[][] edges) {
            long[][] memo = new long[nums.length][2];
            for (long[] row : memo) {
                Arrays.fill(row, -1);
            }
            return maxSumOfNodes(0, 1, nums, k, memo);
        }

        long maxSumOfNodes(int index, int isEven, int[] nums, int k, long[][] memo) {
            if (index == nums.length) {
                return isEven == 1 ? 0 : Integer.MIN_VALUE;
            }
            if (memo[index][isEven] != -1) {
                return memo[index][isEven];
            }

            long noXorDone = nums[index] + maxSumOfNodes(index + 1, isEven, nums, k, memo);
            long xorDone = (nums[index] ^ k) + maxSumOfNodes(index + 1, isEven ^ 1, nums, k, memo);

            return memo[index][isEven] = Math.max(xorDone, noXorDone);
        }
    }

    static class Solution2 {
        public long maximumValueSum(int[] nums, int k, int[][] edges) {
            int n = nums.length;
            long[][] dp = new long[n + 1][2];
            dp[n][1] = 0;
            dp[n][0] = Integer.MIN_VALUE;

            for (int index = n - 1; index >= 0; index--) {
                for (int isEven = 0; isEven <= 1; isEven++) {
                    long performOperation = dp[index + 1][isEven ^ 1] + (nums[index] ^ k);
                    long dontPerformOperation = dp[index + 1][isEven] + nums[index];
                    dp[index][isEven] = Math.max(performOperation, dontPerformOperation);
                }
            }

            return dp[0][1];
        }
    }

}
