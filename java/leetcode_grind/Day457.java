package leetcode_grind;

import java.util.Arrays;

public class Day457 {
    // https://leetcode.com/problems/cherry-pickup/description/
    static class Solution1 {
        int[][][] memo;
        int[][] grid;
        int N;

        public int cherryPickup(int[][] grid) {
            this.grid = grid;
            N = grid.length;
            memo = new int[N][N][N];
            for (var r : memo) {
                for (var r2 : r) {
                    Arrays.fill(r2, Integer.MIN_VALUE);
                }
            }

            return Math.max(0, dp(0, 0, 0));
        }

        public int dp(int r1, int c1, int c2) {
            int r2 = r1 + c1 - c2;
            if (N == r1 || N == r2 || N == c1 || N == c2 || grid[r1][c1] == -1 || grid[r2][c2] == -1) {
                return -999999;
            } else if (r1 == N - 1 && c1 == N - 1) {
                return grid[r1][c1];
            } else if (memo[r1][c1][c2] != Integer.MIN_VALUE) {
                return memo[r1][c1][c2];
            } else {
                int ans = grid[r1][c1];
                if (c1 != c2) {
                    ans += grid[r2][c2];
                }

                ans += Math.max(
                        Math.max(
                                dp(r1, c1 + 1, c2 + 1), dp(r1 + 1, c1, c2 + 1)),
                        Math.max(
                                dp(r1, c1 + 1, c2), dp(r1 + 1, c1, c2)));
                memo[r1][c1][c2] = ans;
                return ans;
            }
        }
    }

    // https://leetcode.com/problems/cherry-pickup-ii/description/
    static class Solution2 {
        int[][] grid;
        int N;
        int M;
        int[][][] memo;

        public int dp(int r, int c1, int c2) {
            if (r == M || c1 == N || c2 == N || 0 > c1 || 0 > c2) {
                return 0;
            }

            if (memo[r][c1][c2] != -1) {
                return memo[r][c1][c2];
            }

            int ans = 0;
            ans += grid[r][c1];
            if (c1 != c2) {
                ans += grid[r][c2];
            }

            if (r != grid.length - 1) {
                int max = 0;

                for (int newCol1 = c1 - 1; newCol1 <= c1 + 1; newCol1++) {
                    for (int newCol2 = c2 - 1; newCol2 <= c2 + 1; newCol2++) {
                        max = Math.max(max, dp(r + 1, newCol1, newCol2));
                    }
                }
                ans += max;
            }
            memo[r][c1][c2] = ans;
            return ans;
        }

        public int cherryPickup(int[][] grid) {
            this.grid = grid;
            this.M = grid.length;
            this.N = grid[0].length;
            this.memo = new int[M][N][N];
            for (var r1 : memo) {
                for (var r2 : r1) {
                    Arrays.fill(r2, -1);
                }
            }
            return dp(0, 0, N - 1);
        }
    }

}
