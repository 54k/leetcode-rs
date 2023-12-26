package leetcode_grind;

public class Day409 {
    // https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/description/
    static class Solution {
        final int MOD = 1_000_000_007;

        int waysToTarget(Integer[][] memo, int diceIndex, int n, int currSum, int target, int k) {
            if (diceIndex == n) {
                return currSum == target ? 1 : 0;
            }

            if (memo[diceIndex][currSum] != null) {
                return memo[diceIndex][currSum];
            }

            int ways = 0;
            for (int i = 1; i <= Math.min(k, target - currSum); i++) {
                ways = (ways + waysToTarget(memo, diceIndex + 1, n, currSum + i, target, k)) % MOD;
            }
            return memo[diceIndex][currSum] = ways;
        }

        public int numRollsToTarget1(int n, int k, int target) {
            Integer[][] memo = new Integer[n + 1][target + 1];
            return waysToTarget(memo, 0, n, 0, target, k);
        }

        public int numRollsToTarget2(int n, int k, int target) {
            int MOD = 1_000_000_007;
            int[][] dp = new int[n + 1][target + 1];
            dp[n][target] = 1;

            for (int i = n - 1; i >= 0; i--) {
                for (int j = 0; j <= target; j++) {
                    int ways = 0;

                    for (int x = 1; x <= Math.min(k, target - j); x++) {
                        ways = (ways + dp[i + 1][x + j]) % MOD;
                    }

                    dp[i][j] = ways;
                }
            }

            return dp[0][0];
        }
    }
}
