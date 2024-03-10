package leetcode_grind;

import java.util.Arrays;

public class Day483 {
    // https://leetcode.com/problems/paint-house-ii/description/
    static class Solution1 {
        public int minCostII(int[][] costs) {
            int n = costs.length;
            int k = costs[0].length;
            int[][] dp = new int[n + 1][k];
            for (int i = 0; i <= n; i++) {
                Arrays.fill(dp[i], 1_000_000_000);
            }
            dp[0] = new int[k];

            for (int i = 1; i <= n; i++) {
                for (int j = 0; j < k; j++) {
                    for (int m = 0; m < k; m++) {
                        if (j == m) {
                            continue;
                        }
                        dp[i][j] = Math.min(costs[i - 1][j] + dp[i - 1][m], dp[i][j]);
                    }
                }
            }
            int ans = 1_000_000_000;
            for (int j = 0; j < k; j++) {
                ans = Math.min(dp[n][j], ans);
            }
            return ans;
        }
    }
}
