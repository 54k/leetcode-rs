package leetcode_grind;

public class Day440 {
    // https://leetcode.com/problems/out-of-boundary-paths/description
    static class Solution1 {
        public int findPaths(int m, int n, int N, int x, int y) {
            int M = 1000000000 + 7;
            long dp[][] = new long[m][n];
            dp[x][y] = 1;

            long count = 0;
            for (int moves = 1; moves <= N; moves++) {
                long[][] temp = new long[m][n];

                for (int i = 0; i < m; i++) {
                    for (int j = 0; j < n; j++) {
                        if (i == 0)
                            count = (count + dp[i][j] % M) % M;
                        if (i == m - 1)
                            count = (count + dp[i][j] % M) % M;
                        if (j == 0)
                            count = (count + dp[i][j] % M) % M;
                        if (j == n - 1)
                            count = (count + dp[i][j] % M) % M;

                        temp[i][j] = (((i > 0) ? dp[i - 1][j] : 0) % M + ((i < m - 1) ? dp[i + 1][j] : 0) % M +
                                ((j > 0) ? dp[i][j - 1] : 0) % M + ((j < n - 1) ? dp[i][j + 1] : 0) % M) % M;
                    }
                }

                dp = temp;
            }

            return (int) count % M;
        }
    }
}