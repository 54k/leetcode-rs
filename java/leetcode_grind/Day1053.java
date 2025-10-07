package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

public class Day1053 {
    // https://leetcode.com/problems/pacific-atlantic-water-flow/description/?envType=daily-question&envId=2025-10-07
    static class Solution1 {
        private static final int[][] DIRECTIONS = new int[][] { { 0, 1 }, { 1, 0 }, { -1, 0 }, { 0, -1 } };
        private int numRows;
        private int numCols;
        private int[][] landHeights;

        public List<List<Integer>> pacificAtlantic(int[][] matrix) {
            if (matrix.length == 0 || matrix[0].length == 0) {
                return new ArrayList<>();
            }

            numRows = matrix.length;
            numCols = matrix[0].length;
            landHeights = matrix;

            Queue<int[]> pacificQueue = new LinkedList<>();
            Queue<int[]> atlanticQueue = new LinkedList<>();
            for (int i = 0; i < numRows; i++) {
                pacificQueue.offer(new int[] { i, 0 });
                atlanticQueue.offer(new int[] { i, numCols - 1 });
            }
            for (int i = 0; i < numCols; i++) {
                pacificQueue.offer(new int[] { 0, i });
                atlanticQueue.offer(new int[] { numRows - 1, i });
            }

            boolean[][] pacificReachable = bfs(pacificQueue);
            boolean[][] atlanticReachable = bfs(atlanticQueue);

            List<List<Integer>> commonCells = new ArrayList<>();
            for (int i = 0; i < numRows; i++) {
                for (int j = 0; j < numCols; j++) {
                    if (pacificReachable[i][j] && atlanticReachable[i][j]) {
                        commonCells.add(List.of(i, j));
                    }
                }
            }
            return commonCells;
        }

        private boolean[][] bfs(Queue<int[]> queue) {
            boolean[][] reachable = new boolean[numRows][numCols];

            while (!queue.isEmpty()) {
                int[] cell = queue.poll();
                reachable[cell[0]][cell[1]] = true;

                for (int[] dir : DIRECTIONS) {
                    int newRow = cell[0] + dir[0];
                    int newCol = cell[1] + dir[1];

                    if (newRow < 0 || newRow >= numRows || newCol < 0 || newCol >= numCols) {
                        continue;
                    }

                    if (reachable[newRow][newCol]) {
                        continue;
                    }

                    if (landHeights[newRow][newCol] < landHeights[cell[0]][cell[1]]) {
                        continue;
                    }

                    queue.offer(new int[] { newRow, newCol });
                }
            }

            return reachable;
        }
    }

    static class Solution2 {
        private static final int[][] DIRECTIONS = new int[][] { { 0, 1 }, { 1, 0 }, { -1, 0 }, { 0, -1 } };
        private int numRows;
        private int numCols;
        private int[][] landHeights;

        public List<List<Integer>> pacificAtlantic(int[][] matrix) {
            if (matrix.length == 0 || matrix[0].length == 0) {
                return new ArrayList<>();
            }

            numRows = matrix.length;
            numCols = matrix[0].length;
            landHeights = matrix;

            boolean[][] pacificReachable = new boolean[numRows][numCols];
            boolean[][] atlanticReachable = new boolean[numRows][numCols];

            for (int i = 0; i < numRows; i++) {
                dfs(i, 0, pacificReachable);
                dfs(i, numCols - 1, atlanticReachable);
            }

            for (int i = 0; i < numCols; i++) {
                dfs(0, i, pacificReachable);
                dfs(numRows - 1, i, atlanticReachable);
            }

            List<List<Integer>> commonCells = new ArrayList<>();
            for (int i = 0; i < numRows; i++) {
                for (int j = 0; j < numCols; j++) {
                    if (pacificReachable[i][j] && atlanticReachable[i][j]) {
                        commonCells.add(List.of(i, j));
                    }
                }
            }
            return commonCells;
        }

        private void dfs(int row, int col, boolean[][] reachable) {
            reachable[row][col] = true;

            for (int[] dir : DIRECTIONS) {
                int newRow = row + dir[0];
                int newCol = col + dir[1];

                if (newRow < 0 || newRow >= numRows || newCol < 0 || newCol >= numCols) {
                    continue;
                }

                if (reachable[newRow][newCol]) {
                    continue;
                }

                if (landHeights[newRow][newCol] < landHeights[row][col]) {
                    continue;
                }

                dfs(newRow, newCol, reachable);
            }
        }
    }
}
