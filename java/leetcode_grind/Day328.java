package leetcode_grind;

public class Day328 {
    // https://leetcode.com/problems/integer-break/description/
    static class Solution {
        public int integerBreak(int n) {
            if (n <= 3) {
                return n - 1;
            }
            var dp = new int[n + 1];
            dp[1] = 1;
            dp[2] = 2;
            dp[3] = 3;

            for (int i = 4; i <= n; i++) {
                for (int j = 1; j <= i; j++) {
                    dp[i] = Math.max(dp[i], j * dp[i - j]);
                }
            }

            return dp[n];
        }
    }
}
