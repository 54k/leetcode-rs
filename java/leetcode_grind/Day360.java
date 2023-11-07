package leetcode_grind;

import java.util.Arrays;

public class Day360 {
    // https://leetcode.com/problems/unique-paths/description
    static class Solution1 {
        public int uniquePaths(int m, int n) {
            var dp = new int[n];
            Arrays.fill(dp, 1);
            for (var i = 1; i < m; i++) {
                var next = new int[n];
                next[0] = 1;
                for (var j = 1; j < n; j++) {
                    next[j] = next[j - 1] + dp[j];
                }
                dp = next;
            }
            return dp[n - 1];
        }
    }

    // https://leetcode.com/problems/minimum-path-sum/description
    static class Solution {
        public int minPathSum(int[][] grid) {
            var m = grid.length;
            var n = grid[0].length;

            for (int i = m - 1; i >= 0; i--) {
                for (int j = n - 1; j >= 0; j--) {
                    if (i == m - 1 && j != n - 1) {
                        grid[i][j] = grid[i][j] + grid[i][j + 1];
                    } else if (j == n - 1 && i != m - 1) {
                        grid[i][j] = grid[i][j] + grid[i + 1][j];
                    } else if (j != n - 1 && i != m - 1) {
                        grid[i][j] = grid[i][j] + Math.min(grid[i + 1][j], grid[i][j + 1]);
                    }
                }
            }

            return grid[0][0];
        }
    }
}
