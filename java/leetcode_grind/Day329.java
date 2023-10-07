package leetcode_grind;

public class Day329 {
    // https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons
    static class Solution {
        public int numOfArraysBottomUp(int n, int m, int k) {
            var dp = new int[n + 1][m + 1][k + 1];
            var MOD = 1000000007;

            for (var num = 0; num < dp[0].length; num++) {
                dp[n][num][0] = 1;
            }

            for (var i = n - 1; i >= 0; i--) {
                for (var maxSoFar = m; maxSoFar >= 0; maxSoFar--) {
                    for (var remain = 0; remain <= k; remain++) {
                        var ans = 0;
                        for (int num = 1; num <= maxSoFar; num++) {
                            ans = (ans + dp[i + 1][maxSoFar][remain]) % MOD;
                        }

                        if (remain > 0) {

                            for (int num = maxSoFar + 1; num <= m; num++) {
                                ans = (ans + dp[i + 1][num][remain - 1]) % MOD;
                            }
                        }

                        dp[i][maxSoFar][remain] = ans;
                    }
                }
            }

            return dp[0][0][k];
        }

        public int numOfArraysPerfixSum(int n, int m, int k) {
            var dp = new long[n + 1][m + 1][k + 1];
            var prefix = new long[n + 1][m + 1][k + 1];
            var MOD = 1000000007;

            for (var num = 1; num <= m; num++) {
                dp[1][num][1] = 1;
                prefix[1][num][1] = prefix[1][num - 1][1] + 1;
            }

            for (var i = 1; i <= n; i++) {
                for (var maxNum = 1; maxNum <= m; maxNum++) {
                    for (var cost = 1; cost <= k; cost++) {
                        long ans = (maxNum * dp[i - 1][maxNum][cost]) % MOD;
                        ans = (ans + prefix[i - 1][maxNum - 1][cost - 1]) % MOD;

                        dp[i][maxNum][cost] += ans;
                        dp[i][maxNum][cost] %= MOD;

                        prefix[i][maxNum][cost] = prefix[i][maxNum - 1][cost] + dp[i][maxNum][cost];

                        prefix[i][maxNum][cost] %= MOD;
                    }
                }
            }

            return (int) prefix[n][m][k];
        }
    }
}
