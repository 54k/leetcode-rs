package leetcode_grind;

public class Day1124 {
    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-v/description/?envType=daily-question&envId=2025-12-17
    static class Solution1 {
        int[] prices;
        long[][][] memo;

        public long maximumProfit(int[] prices, int k) {
            this.prices = prices;
            int n = prices.length;
            memo = new long[n][k + 1][3];
            for (int i = 0; i < n; i++) {
                for (int j = 0; j <= k; j++) {
                    for (int s = 0; s < 3; s++) {
                        memo[i][j][s] = -1;
                    }
                }
            }
            return dfs(n - 1, k, 0);
        }

        long dfs(int i, int j, int state) {
            if (j == 0) {
                return 0;
            }
            if (i == 0) {
                return state == 0 ? 0 : (state == 1 ? -prices[0] : prices[0]);
            }
            if (memo[i][j][state] != -1) {
                return memo[i][j][state];
            }

            int p = prices[i];
            long res;
            if (state == 0) {
                res = Math.max(dfs(i - 1, j, 0), Math.max(dfs(i - 1, j, 1) + p, dfs(i - 1, j, 2) - p));
            } else if (state == 1) {
                res = Math.max(dfs(i - 1, j, 1), dfs(i - 1, j - 1, 0) - p);
            } else {
                res = Math.max(dfs(i - 1, j, 2), dfs(i - 1, j - 1, 0) + p);
            }
            memo[i][j][state] = res;
            return res;
        }
    }
}
