package leetcode_grind;

import java.util.Arrays;
import java.util.LinkedList;
import java.util.Queue;

public class Day795 {
    // https://leetcode.com/problems/map-of-highest-peak/description/?envType=daily-question&envId=2025-01-22
    static class Solution1 {
        public int[][] highestPeak(int[][] isWater) {
            int[] dx = { 0, 0, 1, -1 };
            int[] dy = { 1, -1, 0, 0 };

            int rows = isWater.length;
            int columns = isWater[0].length;

            int[][] cellHeights = new int[rows][columns];
            for (int[] row : cellHeights) {
                Arrays.fill(row, -1);
            }

            Queue<int[]> cellQueue = new LinkedList<>();

            for (int x = 0; x < rows; x++) {
                for (int y = 0; y < columns; y++) {
                    if (isWater[x][y] == 1) {
                        cellQueue.add(new int[] { x, y });
                        cellHeights[x][y] = 0;
                    }
                }
            }

            int heightOfNextLayer = 1;

            while (!cellQueue.isEmpty()) {
                int layerSize = cellQueue.size();

                for (int i = 0; i < layerSize; i++) {
                    int[] currentCell = cellQueue.poll();

                    for (int d = 0; d < 4; d++) {
                        int neighborX = currentCell[0] + dx[d];
                        int neighborY = currentCell[1] + dy[d];

                        if (isValidCell(neighborX, neighborY, rows, columns)
                                && cellHeights[neighborX][neighborY] == -1) {
                            cellHeights[neighborX][neighborY] = heightOfNextLayer;
                            cellQueue.add(new int[] { neighborX, neighborY });
                        }
                    }
                }
                heightOfNextLayer++;
            }

            return cellHeights;
        }

        boolean isValidCell(int x, int y, int rows, int columns) {
            return x >= 0 && y >= 0 && x < rows && y < columns;
        }
    }

    static class Solution2 {
        public int[][] highestPeak(int[][] isWater) {
            int rows = isWater.length;
            int columns = isWater[0].length;
            int INF = rows * columns;

            int[][] cellHeights = new int[rows][columns];
            for (int[] row : cellHeights) {
                Arrays.fill(row, INF);
            }

            for (int row = 0; row < rows; row++) {
                for (int col = 0; col < columns; col++) {
                    if (isWater[row][col] == 1) {
                        cellHeights[row][col] = 0;
                    }
                }
            }

            for (int row = 0; row < rows; row++) {
                for (int col = 0; col < columns; col++) {
                    int minNeighborDistane = INF;

                    int neighborRow = row - 1;
                    int neighborCol = col;

                    if (isValidCell(neighborRow, neighborCol, rows, columns)) {
                        minNeighborDistane = Math.min(minNeighborDistane, cellHeights[neighborRow][neighborCol]);
                    }

                    neighborRow = row;
                    neighborCol = col - 1;

                    if (isValidCell(neighborRow, neighborCol, rows, columns)) {
                        minNeighborDistane = Math.min(minNeighborDistane, cellHeights[neighborRow][neighborCol]);
                    }

                    cellHeights[row][col] = Math.min(cellHeights[row][col], minNeighborDistane + 1);
                }
            }

            for (int row = rows - 1; row >= 0; row--) {
                for (int col = columns - 1; col >= 0; col--) {
                    int minNeighborDistane = INF;

                    int neighborRow = row + 1;
                    int neighborCol = col;

                    if (isValidCell(neighborRow, neighborCol, rows, columns)) {
                        minNeighborDistane = Math.min(minNeighborDistane, cellHeights[neighborRow][neighborCol]);
                    }

                    neighborRow = row;
                    neighborCol = col + 1;

                    if (isValidCell(neighborRow, neighborCol, rows, columns)) {
                        minNeighborDistane = Math.min(minNeighborDistane, cellHeights[neighborRow][neighborCol]);
                    }

                    cellHeights[row][col] = Math.min(cellHeights[row][col], minNeighborDistane + 1);
                }
            }

            return cellHeights;
        }

        boolean isValidCell(int row, int col, int rows, int cols) {
            return row >= 0 && col >= 0 && row < rows && col < cols;
        }
    }

    // https://leetcode.com/problems/sum-of-remoteness-of-all-cells/description/?envType=weekly-question&envId=2025-01-22
    static class Solution3 {

        int[][] dir = { { 0, 1 }, { 0, -1 }, { 1, 0 }, { -1, 0 } };

        public long sumRemoteness(int[][] grid) {
            int n = grid.length;
            long totalSum = 0;
            for (int row = 0; row < n; row++) {
                for (int col = 0; col < n; col++) {
                    if (grid[row][col] != -1) {
                        totalSum += grid[row][col];
                    }
                }
            }

            long result = 0;

            for (int row = 0; row < n; row++) {
                for (int col = 0; col < n; col++) {
                    if (grid[row][col] > 0) {
                        long[] arr = new long[2];
                        dfs(grid, row, col, arr);
                        result += (totalSum - arr[0]) * arr[1];
                    }
                }
            }
            return result;
        }

        void dfs(int[][] grid, int row, int col, long[] arr) {
            arr[0] += grid[row][col];
            arr[1]++;
            grid[row][col] = -1;

            for (int[] d : dir) {
                int newRow = row + d[0];
                int newCol = col + d[1];
                if (isValid(grid, newRow, newCol)) {
                    dfs(grid, newRow, newCol, arr);
                }
            }
        }

        boolean isValid(int[][] grid, int row, int col) {
            int n = grid.length;
            return row >= 0 && col >= 0 && row < n && col < n && grid[row][col] > 0;
        }
    }

    static class Solution4 {

        int[][] dir = { { 0, 1 }, { 0, -1 }, { 1, 0 }, { -1, 0 } };

        public long sumRemoteness(int[][] grid) {
            int n = grid.length;
            long totalSum = 0;
            for (int row = 0; row < n; row++) {
                for (int col = 0; col < n; col++) {
                    if (grid[row][col] != -1) {
                        totalSum += grid[row][col];
                    }
                }
            }

            long result = 0;

            for (int row = 0; row < n; row++) {
                for (int col = 0; col < n; col++) {
                    if (grid[row][col] > 0) {
                        result += bfs(grid, row, col, totalSum);
                    }
                }
            }
            return result;
        }

        long bfs(int[][] grid, int row, int col, long totalSum) {
            long compSum = grid[row][col];
            long compSize = 1;
            grid[row][col] = -1;

            Queue<int[]> queue = new LinkedList<>();
            queue.add(new int[] { row, col });

            while (!queue.isEmpty()) {
                int[] curr = queue.poll();

                for (int[] d : dir) {
                    int newRow = d[0] + curr[0];
                    int newCol = d[1] + curr[1];
                    if (isValid(grid, newRow, newCol)) {
                        queue.add(new int[] { newRow, newCol });
                        compSum += grid[newRow][newCol];
                        compSize++;
                        grid[newRow][newCol] = -1;
                    }
                }
            }
            return (totalSum - compSum) * compSize;
        }

        boolean isValid(int[][] grid, int row, int col) {
            int n = grid.length;
            return row >= 0 && col >= 0 && row < n && col < n && grid[row][col] > 0;
        }
    }
}
