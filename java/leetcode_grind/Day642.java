package leetcode_grind;

import java.util.Arrays;

public class Day642 {
    // https://leetcode.com/problems/stone-game-ii/description/?envType=daily-question&envId=2024-08-20
    static class Solution1 {
        public int stoneGameII(int[] piles) {
            int[] suffixSum = Arrays.copyOf(piles, piles.length);
            for (int i = suffixSum.length - 2; i >= 0; i--) {
                suffixSum[i] += suffixSum[i + 1];
            }
            return maxStones(suffixSum, 1, 0, new int[piles.length][piles.length]);
        }

        int maxStones(int[] suffixSum, int maxTillNow, int currIndex, int[][] memo) {
            if (currIndex + 2 * maxTillNow >= suffixSum.length) {
                return suffixSum[currIndex];
            }
            if (memo[currIndex][maxTillNow] > 0)
                return memo[currIndex][maxTillNow];
            int res = Integer.MAX_VALUE;
            for (int i = 1; i <= 2 * maxTillNow; i++) {
                res = Math.min(
                        res,
                        maxStones(
                                suffixSum, Math.max(i, maxTillNow), currIndex + i, memo));
            }

            memo[currIndex][maxTillNow] = suffixSum[currIndex] - res;
            return memo[currIndex][maxTillNow];
        }
    }

    static class Solution2 {
        public int stoneGameII(int[] piles) {
            int length = piles.length;
            int[][] dp = new int[length + 1][length + 1];
            int[] suffixSum = new int[length + 1];
            for (int i = length - 1; i >= 0; i--) {
                suffixSum[i] = suffixSum[i + 1] + piles[i];
            }
            for (int i = 0; i <= length; i++) {
                dp[i][length] = suffixSum[i];
            }

            for (int index = length - 1; index >= 0; index--) {
                for (int maxTillNow = length - 1; maxTillNow >= 1; maxTillNow--) {
                    for (int X = 1; X <= 2 * maxTillNow && index + X <= length; X++) {
                        dp[index][maxTillNow] = Math.max(
                                dp[index][maxTillNow],
                                suffixSum[index] - dp[index + X][Math.max(maxTillNow, X)]);
                    }
                }
            }
            return dp[0][1];
        }
    }
}
