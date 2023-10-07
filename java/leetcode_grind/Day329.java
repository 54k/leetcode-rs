package leetcode_grind;

public class Day329 {
    // https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons
    static class Solution {
        public int numOfArrays(int n, int m, int k) {
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
    }
}
