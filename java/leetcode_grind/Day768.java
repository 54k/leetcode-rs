package leetcode_grind;

import java.util.Arrays;

public class Day768 {
    // https://leetcode.com/problems/target-sum/description/
    static class Solution1 {
        int totalWays = 0;

        public int findTargetSumWays(int[] nums, int target) {
            calculateWays(nums, 0, 0, target);
            return totalWays;
        }

        void calculateWays(int[] nums, int currentIndex, int currentSum, int target) {
            if (currentIndex == nums.length) {
                if (currentSum == target)
                    totalWays++;
            } else {
                calculateWays(nums, currentIndex + 1, currentSum + nums[currentIndex], target);
                calculateWays(nums, currentIndex + 1, currentSum - nums[currentIndex], target);
            }
        }
    }

    static class Solution2 {
        int totalSum = 0;

        public int findTargetSumWays(int[] nums, int target) {
            totalSum = Arrays.stream(nums).sum();
            int[][] memo = new int[nums.length][2 * totalSum + 1];
            for (int[] row : memo) {
                Arrays.fill(row, Integer.MIN_VALUE);
            }
            return calculateWays(nums, 0, 0, target, memo);
        }

        int calculateWays(int[] nums, int currentIndex, int currentSum, int target, int[][] memo) {
            if (currentIndex == nums.length) {
                if (currentSum == target)
                    return 1;
                else
                    return 0;
            } else {
                if (memo[currentIndex][currentSum + totalSum] != Integer.MIN_VALUE) {
                    return memo[currentIndex][currentSum + totalSum];
                }

                int add = calculateWays(nums, currentIndex + 1, currentSum + nums[currentIndex], target, memo);
                int substract = calculateWays(nums, currentIndex + 1, currentSum - nums[currentIndex], target, memo);
                memo[currentIndex][currentSum + totalSum] = add + substract;
            }
            return memo[currentIndex][currentSum + totalSum];
        }
    }

    static class Solution3 {
        public int findTargetSumWays(int[] nums, int target) {
            int totalSum = Arrays.stream(nums).sum();
            int[][] dp = new int[nums.length][2 * totalSum + 1];
            dp[0][nums[0] + totalSum] = 1;
            dp[0][-nums[0] + totalSum] += 1;

            for (int index = 1; index < nums.length; index++) {
                for (int sum = -totalSum; sum <= totalSum; sum++) {
                    if (dp[index - 1][sum + totalSum] > 0) {
                        dp[index][sum + nums[index] + totalSum] += dp[index - 1][sum + totalSum];
                        dp[index][sum - nums[index] + totalSum] += dp[index - 1][sum + totalSum];
                    }
                }
            }

            return Math.abs(target) > totalSum ? 0 : dp[nums.length - 1][target + totalSum];
        }
    }

}
