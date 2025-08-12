package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

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

    // https://leetcode.com/problems/longest-increasing-path-in-a-matrix/description/
    static class Solution3 {
        static final int[][] dirs = { { 0, 1 }, { 1, 0 }, { 0, -1 }, { -1, 0 } };
        int m, n;

        public int longestIncreasingPath(int[][] matrix) {
            if (matrix.length == 0)
                return 0;
            m = matrix.length;
            n = matrix[0].length;
            int ans = 0;
            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    ans = Math.max(ans, dfs(matrix, i, j));
                }
            }
            return ans;
        }

        int dfs(int[][] matrix, int i, int j) {
            int ans = 0;
            for (int[] d : dirs) {
                int x = i + d[0], y = j + d[1];
                if (0 <= x && x < m && 0 <= y && y < n && matrix[x][y] > matrix[i][j]) {
                    ans = Math.max(ans, dfs(matrix, x, y));
                }
            }
            return ++ans;
        }
    }

    static class Solution4 {
        static final int[][] dirs = { { 0, 1 }, { 1, 0 }, { 0, -1 }, { -1, 0 } };
        int m, n;

        public int longestIncreasingPath(int[][] matrix) {
            if (matrix.length == 0)
                return 0;
            m = matrix.length;
            n = matrix[0].length;
            int[][] cache = new int[m][n];
            int ans = 0;
            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    ans = Math.max(ans, dfs(matrix, i, j, cache));
                }
            }
            return ans;
        }

        int dfs(int[][] matrix, int i, int j, int[][] cache) {
            if (cache[i][j] != 0)
                return cache[i][j];
            for (int[] d : dirs) {
                int x = i + d[0], y = j + d[1];
                if (0 <= x && x < m && 0 <= y && y < n && matrix[x][y] > matrix[i][j]) {
                    cache[i][j] = Math.max(cache[i][j], dfs(matrix, x, y, cache));
                }
            }
            return ++cache[i][j];
        }
    }

    // Topological Sort Based Solution
    // An Alternative Solution
    static public class Solution5 {
        private static final int[][] dir = { { 0, 1 }, { 1, 0 }, { 0, -1 }, { -1, 0 } };
        private int m, n;

        public int longestIncreasingPath(int[][] grid) {
            int m = grid.length;
            if (m == 0)
                return 0;
            int n = grid[0].length;
            // padding the matrix with zero as boundaries
            // assuming all positive integer, otherwise use INT_MIN as boundaries
            int[][] matrix = new int[m + 2][n + 2];
            for (int i = 0; i < m; ++i)
                System.arraycopy(grid[i], 0, matrix[i + 1], 1, n);

            // calculate outdegrees
            int[][] outdegree = new int[m + 2][n + 2];
            for (int i = 1; i <= m; ++i)
                for (int j = 1; j <= n; ++j)
                    for (int[] d : dir)
                        if (matrix[i][j] < matrix[i + d[0]][j + d[1]])
                            outdegree[i][j]++;

            // find leaves who have zero out degree as the initial level
            n += 2;
            m += 2;
            List<int[]> leaves = new ArrayList<>();
            for (int i = 1; i < m - 1; ++i)
                for (int j = 1; j < n - 1; ++j)
                    if (outdegree[i][j] == 0)
                        leaves.add(new int[] { i, j });

            // remove leaves level by level in topological order
            int height = 0;
            while (!leaves.isEmpty()) {
                height++;
                List<int[]> newLeaves = new ArrayList<>();
                for (int[] node : leaves) {
                    for (int[] d : dir) {
                        int x = node[0] + d[0], y = node[1] + d[1];
                        if (matrix[node[0]][node[1]] > matrix[x][y])
                            if (--outdegree[x][y] == 0)
                                newLeaves.add(new int[] { x, y });
                    }
                }
                leaves = newLeaves;
            }
            return height;
        }
    }
}
