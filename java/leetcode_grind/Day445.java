package leetcode_grind;

public class Day445 {
    // https://leetcode.com/problems/paint-house/
    static class Solution1 {
        public int minCost(int[][] costs) {
            var n = costs.length;
            var dp = new int[n + 1][3];
            dp[1] = costs[0];

            for (int i = 2; i <= n; i++) {
                for (int j = 0; j < 3; j++) {
                    dp[i][j] = costs[i - 1][j] + Math.min(dp[i - 1][(j + 1) % 3], dp[i - 1][(j + 2) % 3]);
                }
            }

            var min = Integer.MAX_VALUE;
            for (var v : dp[n]) {
                min = Math.min(min, v);
            }

            return min;
        }
    }
}
