package leetcode_grind;

public class Day336 {
    // https://leetcode.com/problems/painting-the-walls/description
    static class Solution {
        public int paintWalls(int[] cost, int[] time) {
            var n = cost.length;
            var dp = new int[n + 1][n + 1];
            for (int i = 1; i < dp.length; i++) {
                dp[n][i] = 1000_000_000;
            }

            for (int i = n - 1; i >= 0; i--) {
                for (int j = 1; j <= n; j++) {
                    var paint = cost[i] + dp[i + 1][Math.max(0, j - time[i] - 1)];
                    var notPaint = dp[i + 1][j];
                    dp[i][j] = Math.min(paint, notPaint);
                }
            }

            return dp[0][n];
        }
    }
}
