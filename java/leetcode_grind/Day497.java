package leetcode_grind;

import java.util.List;

public class Day497 {
    // https://leetcode.com/problems/maximum-value-of-k-coins-from-piles/description/s
    static class Solution1 {
        public int maxValueOfCoins(List<List<Integer>> piles, int k) {
            int n = piles.size();
            int[][] dp = new int[n + 1][k + 1];
            for (int i = 1; i <= n; i++) {
                for (int coins = 0; coins <= k; coins++) {
                    int curSum = 0;
                    for (int curCoins = 0; curCoins <= Math.min(piles.get(i - 1).size(), coins); curCoins++) {
                        if (curCoins > 0) {
                            curSum += piles.get(i - 1).get(curCoins - 1);
                        }
                        dp[i][coins] = Math.max(dp[i][coins], dp[i - 1][coins - curCoins] + curSum);
                    }
                }
            }
            return dp[n][k];
        }
    }
}
