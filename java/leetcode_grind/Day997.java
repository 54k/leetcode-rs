package leetcode_grind;

public class Day997 {
    // https://leetcode.com/problems/ways-to-express-an-integer-as-sum-of-powers/description/?envType=daily-question&envId=2025-08-12
    static class Solution1 {
        int MOD = 1_000_000_007;

        public int numberOfWays(int n, int x) {
            long[][] dp = new long[n + 1][n + 1];
            dp[0][0] = 1;
            for (int i = 1; i <= n; i++) {
                long val = (long) Math.pow(i, x);
                for (int j = 0; j <= n; j++) {
                    dp[i][j] = dp[i - 1][j];
                    if (j >= val) {
                        dp[i][j] = (dp[i][j] + dp[i - 1][j - (int) val]) % MOD;
                    }
                }
            }
            return (int) dp[n][n];
        }
    }

    static class Solution2 {
        int MOD = 1_000_000_007;

        public int numberOfWays(int n, int x) {
            long[] dp = new long[n + 1];
            dp[0] = 1;
            for (int i = 1; i <= n; i++) {
                int val = (int) Math.pow(i, x);
                if (val > n) {
                    break;
                }
                for (int j = n; j >= val; j--) {
                    dp[j] = (dp[j] + dp[j - val]) % MOD;
                }
            }
            return (int) dp[n];
        }
    }

}
