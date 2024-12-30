package leetcode_grind;

import java.util.Arrays;

public class Day772 {
    // https://leetcode.com/problems/count-ways-to-build-good-strings/description/?envType=daily-question&envId=2024-12-30
    static class Solution1 {
        int[] dp;
        int mod = 1_000_000_007;

        int dfs(int end, int zero, int one) {
            if (dp[end] != -1) {
                return dp[end];
            }

            int count = 0;
            if (end >= one) {
                count += dfs(end - one, zero, one);
            }
            if (end >= zero) {
                count += dfs(end - zero, zero, one);
            }
            dp[end] = count % mod;
            return dp[end];
        }

        public int countGoodStrings(int low, int high, int zero, int one) {
            dp = new int[high + 1];
            Arrays.fill(dp, -1);
            dp[0] = 1;

            int answer = 0;
            for (int end = low; end <= high; ++end) {
                answer += dfs(end, zero, one);
                answer %= mod;
            }
            return answer;
        }
    }

    // https://leetcode.com/problems/maximum-subarray-sum-after-one-operation/description/?envType=weekly-question&envId=2024-12-29
    static class Solution2 {
        public int maxSumAfterOperation(int[] nums) {
            int n = nums.length;
            int[] maxLeft = new int[n];
            int[] maxRight = new int[n];
            maxLeft[0] = 0;
            for (int i = 1; i < n; i++) {
                maxLeft[i] = Math.max(maxLeft[i - 1] + nums[i - 1], 0);
            }

            maxRight[n - 1] = 0;
            for (int i = n - 2; i >= 0; i--) {
                maxRight[i] = Math.max(maxRight[i + 1] + nums[i + 1], 0);
            }

            int maxSum = 0;
            for (int i = 0; i < n; i++) {
                maxSum = Math.max(maxSum, maxLeft[i] + nums[i] * nums[i] + maxRight[i]);
            }
            return maxSum;
        }
    }

    static class Solution3 {
        public int maxSumAfterOperation(int[] nums) {
            int n = nums.length;
            // Initialize a DP table to store results of subproblems.
            int[][] dp = new int[n][2];
            for (int i = 0; i < n; i++) {
                Arrays.fill(dp[i], -1); // Initialize all entries to -1.
            }

            // Create array to pass be reference
            int maxSum[] = new int[1];
            maxSum[0] = 0;

            // Call the recursive helper function to compute the result.
            getMaxSumHelper(0, nums, true, dp, maxSum);
            return maxSum[0];
        }

        private int getMaxSumHelper(
                int index,
                int[] nums,
                boolean canSquare,
                int[][] dp,
                int[] maxSum) {
            if (index == nums.length) {
                return 0; // Base case: if we reach the end of the array, return 0.
            }

            // If the result is already computed for this state, return it.
            if (dp[index][canSquare ? 1 : 0] != -1) {
                return dp[index][canSquare ? 1 : 0];
            }

            // Case 1: Skip squaring the current element.
            int nextSumWithoutSquare = getMaxSumHelper(
                    index + 1,
                    nums,
                    canSquare,
                    dp,
                    maxSum);
            int maxSumWithoutSquare = nums[index]; // The value itself if we don't square it.
            if (nextSumWithoutSquare > 0) {
                maxSumWithoutSquare += nextSumWithoutSquare; // Accumulate if positive.
            }

            // Case 2: Square the current element if allowed.
            int maxSumWithSquare = 0;
            if (canSquare) {
                maxSumWithSquare = nums[index] * nums[index]; // Square the current element.
                int nextSumWithSquare = getMaxSumHelper(
                        index + 1,
                        nums,
                        false,
                        dp,
                        maxSum); // Don't square further.
                if (nextSumWithSquare > 0) {
                    maxSumWithSquare += nextSumWithSquare; // Accumulate if positive.
                }
            }

            // Update the global maxSum if we find a better one.
            maxSum[0] = Math.max(
                    maxSum[0],
                    Math.max(maxSumWithSquare, maxSumWithoutSquare));

            // Store the result in dp table and return the maximum of the two options.
            dp[index][canSquare ? 1 : 0] = Math.max(
                    maxSumWithSquare,
                    maxSumWithoutSquare);
            return dp[index][canSquare ? 1 : 0];
        }
    }

    static class Solution4 {
        public int maxSumAfterOperation(int[] nums) {
            int n = nums.length;
            int[][] dp = new int[n][2];
            dp[0][0] = nums[0];
            dp[0][1] = nums[0] * nums[0];

            int maxSum = dp[0][1];
            for (int index = 1; index < n; index++) {
                dp[index][0] = Math.max(nums[index], dp[index - 1][0] + nums[index]);
                dp[index][1] = Math.max(
                        Math.max(nums[index] * nums[index], dp[index - 1][0] + nums[index] * nums[index]),
                        dp[index - 1][1] + nums[index]);
                maxSum = Math.max(maxSum, dp[index][1]);
            }
            return maxSum;
        }
    }
}
