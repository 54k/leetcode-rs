package leetcode_grind;

import java.util.*;

public class Day770 {
    // https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays/description/?envType=daily-question&envId=2024-12-28
    static class Solution1 {
        public int[] maxSumOfThreeSubarrays(int[] nums, int k) {
            int n = nums.length - k + 1;

            int[] sums = new int[n];
            int windowSum = 0;
            for (int i = 0; i < k; i++) {
                windowSum += nums[i];
            }
            sums[0] = windowSum;

            for (int i = k; i < nums.length; i++) {
                windowSum = windowSum - nums[i - k] + nums[i];
                sums[i - k + 1] = windowSum;
            }

            int[][] memo = new int[n][4];
            for (int[] row : memo) {
                Arrays.fill(row, -1);
            }
            dp(sums, k, 0, 3, memo);

            List<Integer> indices = new ArrayList<>();
            dfs(sums, k, 0, 3, memo, indices);

            int[] result = new int[3];
            for (int i = 0; i < 3; i++) {
                result[i] = indices.get(i);
            }
            return result;
        }

        int dp(int[] sums, int k, int idx, int rem, int[][] memo) {
            if (rem == 0)
                return 0;
            if (idx >= sums.length) {
                return rem > 0 ? Integer.MIN_VALUE : 0;
            }
            if (memo[idx][rem] != -1) {
                return memo[idx][rem];
            }

            int withCurrent = sums[idx] + dp(sums, k, idx + k, rem - 1, memo);
            int skipCurrent = dp(sums, k, idx + 1, rem, memo);
            memo[idx][rem] = Math.max(withCurrent, skipCurrent);
            return memo[idx][rem];
        }

        void dfs(
                int[] sums,
                int k,
                int idx,
                int rem,
                int[][] memo,
                List<Integer> indices) {
            if (rem == 0)
                return;
            if (idx >= sums.length)
                return;

            int withCurrent = sums[idx] + dp(sums, k, idx + k, rem - 1, memo);
            int skipCurrent = dp(sums, k, idx + 1, rem, memo);

            if (withCurrent >= skipCurrent) {
                indices.add(idx);
                dfs(sums, k, idx + k, rem - 1, memo, indices);
            } else {
                dfs(sums, k, idx + 1, rem, memo, indices);
            }
        }
    }

    static class Solution2 {
        public int[] maxSumOfThreeSubarrays(int[] nums, int k) {
            int n = nums.length;
            int[] prefixSum = new int[n + 1];
            for (int i = 1; i <= n; i++) {
                prefixSum[i] = prefixSum[i - 1] + nums[i - 1];
            }

            int[][] bestSum = new int[4][n + 1];
            int[][] bestIndex = new int[4][n + 1];

            for (int subarrayCount = 1; subarrayCount <= 3; subarrayCount++) {
                for (int endIndex = k * subarrayCount; endIndex <= n; endIndex++) {
                    int currentSum = prefixSum[endIndex] - prefixSum[endIndex - k]
                            + bestSum[subarrayCount - 1][endIndex - k];
                    if (currentSum > bestSum[subarrayCount][endIndex - 1]) {
                        bestSum[subarrayCount][endIndex] = currentSum;
                        bestIndex[subarrayCount][endIndex] = endIndex - k;
                    } else {
                        bestSum[subarrayCount][endIndex] = bestSum[subarrayCount][endIndex - 1];
                        bestIndex[subarrayCount][endIndex] = bestIndex[subarrayCount][endIndex - 1];
                    }
                }
            }

            int[] result = new int[3];
            int currentEnd = n;
            for (int subarrayIndex = 3; subarrayIndex >= 1; subarrayIndex--) {
                result[subarrayIndex - 1] = bestIndex[subarrayIndex][currentEnd];
                currentEnd = result[subarrayIndex - 1];
            }
            return result;

        }
    }

}
