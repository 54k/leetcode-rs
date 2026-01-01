package leetcode_grind;

import java.util.LinkedList;
import java.util.Queue;

public class Day1138 {
    // https://leetcode.com/problems/last-day-where-you-can-still-cross/description/?envType=daily-question&envId=2025-12-31
    static class Solution1 {
        private int[][] directions = new int[][] { { 1, 0 }, { -1, 0 }, { 0, 1 }, { 0, -1 } };

        public boolean canCross(int row, int col, int[][] cells, int day) {
            int[][] grid = new int[row][col];
            Queue<int[]> queue = new LinkedList<>();

            for (int i = 0; i < day; i++) {
                grid[cells[i][0] - 1][cells[i][1] - 1] = 1;
            }

            for (int i = 0; i < col; i++) {
                if (grid[0][i] == 0) {
                    queue.offer(new int[] { 0, i });
                    grid[0][i] = -1;
                }
            }

            while (!queue.isEmpty()) {
                int[] cur = queue.poll();
                int r = cur[0], c = cur[1];
                if (r == row - 1) {
                    return true;
                }

                for (int[] dir : directions) {
                    int newRow = r + dir[0];
                    int newCol = c + dir[1];
                    if (newRow >= 0 && newRow < row && newCol >= 0 && newCol < col && grid[newRow][newCol] == 0) {
                        grid[newRow][newCol] = -1;
                        queue.offer(new int[] { newRow, newCol });
                    }
                }
            }

            return false;
        }

        public int latestDayToCross(int row, int col, int[][] cells) {
            int left = 1;
            int right = row * col;

            while (left < right) {
                int mid = right - (right - left) / 2;
                if (canCross(row, col, cells, mid)) {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }

            return left;
        }
    }

    static class Solution2 {
        private int[][] directions = new int[][] { { 1, 0 }, { -1, 0 }, { 0, 1 }, { 0, -1 } };

        public boolean canCross(int row, int col, int[][] cells, int day) {
            int[][] grid = new int[row][col];

            for (int i = 0; i < day; i++) {
                grid[cells[i][0] - 1][cells[i][1] - 1] = 1;
            }

            for (int i = 0; i < col; i++) {
                if (grid[0][i] == 0 && dfs(grid, 0, i, row, col)) {
                    return true;
                }
            }

            return false;
        }

        private boolean dfs(int[][] grid, int r, int c, int row, int col) {
            if (r < 0 || r >= row || c < 0 || c >= col || grid[r][c] != 0) {
                return false;
            }

            if (r == row - 1) {
                return true;
            }

            grid[r][c] = -1;
            for (int[] dir : directions) {
                int newR = r + dir[0], newC = c + dir[1];
                if (dfs(grid, newR, newC, row, col)) {
                    return true;
                }
            }

            return false;
        }

        public int latestDayToCross(int row, int col, int[][] cells) {
            int left = 1;
            int right = row * col;
            while (left < right) {
                int mid = right - (right - left) / 2;
                if (canCross(row, col, cells, mid)) {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }
            return left;
        }
    }

    // https://leetcode.com/problems/plus-one/description/?envType=daily-question&envId=2026-01-01
    static class Solution1 {
        public int[] plusOne(int[] digits) {
            for (int i = digits.length - 1; i >= 0; i--) {
                if (digits[i] == 9) {
                    digits[i] = 0;
                } else {
                    digits[i] += 1;
                    return digits;
                }
            }

            var ans = new int[digits.length + 1];
            ans[0] = 1;
            return ans;
        }
    }
}
