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

}
