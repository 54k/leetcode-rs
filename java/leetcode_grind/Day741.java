package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.PriorityQueue;

public class Day741 {
    // https://leetcode.com/problems/minimum-obstacle-removal-to-reach-corner/description/
    static class Solution1 {
        final int[][] directions = {
                { 0, 1 },
                { 0, -1 },
                { 1, 0 },
                { -1, 0 },
        };

        public int minimumObstacles(int[][] grid) {
            int m = grid.length, n = grid[0].length;
            int[][] minObstacles = new int[m][n];
            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    minObstacles[i][j] = Integer.MAX_VALUE;
                }
            }
            minObstacles[0][0] = grid[0][0];
            PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[0] - b[0]);
            pq.add(new int[] { minObstacles[0][0], 0, 0 });

            while (!pq.isEmpty()) {
                int[] current = pq.poll();
                int obstacles = current[0], row = current[1], col = current[2];
                if (row == m - 1 && col == n - 1) {
                    return obstacles;
                }
                for (int[] dir : directions) {
                    int newRow = row + dir[0], newCol = col + dir[1];

                    if (isValid(grid, newRow, newCol)) {
                        int newObstacles = obstacles + grid[newRow][newCol];
                        if (newObstacles < minObstacles[newRow][newCol]) {
                            minObstacles[newRow][newCol] = newObstacles;
                            pq.add(new int[] { newObstacles, newRow, newCol });
                        }
                    }
                }
            }
            return minObstacles[m - 1][n - 1];
        }

        boolean isValid(int[][] grid, int row, int col) {
            return (row >= 0 && col >= 0 && row < grid.length && col < grid[0].length);
        }
    }

    static class Solution2 {
        final int[][] directions = {
                { 0, 1 },
                { 0, -1 },
                { 1, 0 },
                { -1, 0 },
        };

        public int minimumObstacles(int[][] grid) {
            int m = grid.length, n = grid[0].length;
            int[][] minObstacles = new int[m][n];
            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    minObstacles[i][j] = Integer.MAX_VALUE;
                }
            }
            minObstacles[0][0] = 0;
            Deque<int[]> deque = new ArrayDeque<>();
            deque.add(new int[] { 0, 0, 0 });

            while (!deque.isEmpty()) {
                int[] current = deque.poll();
                int obstacles = current[0], row = current[1], col = current[2];

                for (int[] dir : directions) {
                    int newRow = row + dir[0], newCol = col + dir[1];

                    if (isValid(grid, newRow, newCol) && minObstacles[newRow][newCol] == Integer.MAX_VALUE) {
                        if (grid[newRow][newCol] == 1) {
                            minObstacles[newRow][newCol] = obstacles + 1;
                            deque.addLast(new int[] { obstacles + 1, newRow, newCol });
                        } else {
                            minObstacles[newRow][newCol] = obstacles;
                            deque.addFirst(new int[] { obstacles, newRow, newCol });
                        }
                    }
                }
            }

            return minObstacles[m - 1][n - 1];
        }

        boolean isValid(int[][] grid, int row, int col) {
            return (row >= 0 && col >= 0 && row < grid.length && col < grid[0].length);
        }
    }

}
