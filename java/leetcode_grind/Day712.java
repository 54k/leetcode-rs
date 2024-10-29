package leetcode_grind;

import java.util.*;

public class Day712 {
    // https://leetcode.com/problems/maximum-number-of-moves-in-a-grid/description/?envType=daily-question&envId=2024-10-29
    static class Solution1 {
        int[] dirs = { -1, 0, 1 };

        public int maxMoves(int[][] grid) {
            int M = grid.length, N = grid[0].length;
            Queue<int[]> q = new LinkedList<>();
            boolean[][] vis = new boolean[M][N];

            for (int i = 0; i < M; i++) {
                vis[i][0] = true;
                q.offer(new int[] { i, 0, 0 });
            }

            int maxMoves = 0;
            while (!q.isEmpty()) {
                int sz = q.size();
                while (sz-- > 0) {
                    int[] v = q.poll();
                    int row = v[0], col = v[1], count = v[2];
                    maxMoves = Math.max(maxMoves, count);

                    for (int dir : dirs) {
                        int newRow = row + dir, newCol = col + 1;

                        if (newRow >= 0 && newCol >= 0 && newRow < M && newCol < N && !vis[newRow][newCol]
                                && grid[row][col] < grid[newRow][newCol]) {
                            vis[newRow][newCol] = true;
                            q.offer(new int[] { newRow, newCol, count + 1 });
                        }
                    }
                }
            }
            return maxMoves;
        }
    }

    static class Solution2 {
        final int[] dirs = { -1, 0, 1 };

        int DFS(int row, int col, int[][] grid, int[][] dp) {
            int M = grid.length, N = grid[0].length;
            if (dp[row][col] != -1) {
                return dp[row][col];
            }
            int maxMoves = 0;
            for (int dir : dirs) {
                int newRow = row + dir, newCol = col + 1;

                if (newRow >= 0 &&
                        newCol >= 0 &&
                        newRow < M &&
                        newCol < N &&
                        grid[row][col] < grid[newRow][newCol]) {
                    maxMoves = Math.max(
                            maxMoves,
                            1 + DFS(newRow, newCol, grid, dp));
                }
            }

            dp[row][col] = maxMoves;
            return maxMoves;
        }

        public int maxMoves(int[][] grid) {
            int M = grid.length, N = grid[0].length;
            int[][] dp = new int[M][N];
            for (int i = 0; i < M; i++) {
                Arrays.fill(dp[i], -1);
            }
            int maxMoves = 0;
            for (int i = 0; i < M; i++) {
                int movesRequired = DFS(i, 0, grid, dp);
                maxMoves = Math.max(maxMoves, movesRequired);
            }
            return maxMoves;
        }
    }
}
