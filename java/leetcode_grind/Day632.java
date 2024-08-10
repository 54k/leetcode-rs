package leetcode_grind;

import java.util.Arrays;
import java.util.LinkedList;
import java.util.Queue;

public class Day632 {
    // https://leetcode.com/problems/regions-cut-by-slashes/description/?envType=daily-question&envId=2024-08-10
    static class Solution1 {
        private static final int[][] DIRECTIONS = {
                { 0, 1 },
                { 0, -1 },
                { 1, 0 },
                { -1, 0 },
        };

        public int regionsBySlashes(String[] grid) {
            int gridSize = grid.length;
            int[][] expandedGrid = new int[gridSize * 3][gridSize * 3];

            for (int i = 0; i < gridSize; i++) {
                for (int j = 0; j < gridSize; j++) {
                    int baseRow = i * 3;
                    int baseCol = j * 3;

                    if (grid[i].charAt(j) == '\\') {
                        expandedGrid[baseRow][baseCol] = 1;
                        expandedGrid[baseRow + 1][baseCol + 1] = 1;
                        expandedGrid[baseRow + 2][baseCol + 2] = 1;
                    } else if (grid[i].charAt(j) == '/') {
                        expandedGrid[baseRow][baseCol + 2] = 1;
                        expandedGrid[baseRow + 1][baseCol + 1] = 1;
                        expandedGrid[baseRow + 2][baseCol] = 1;
                    }
                }
            }

            int regionCount = 0;
            for (int i = 0; i < gridSize * 3; i++) {
                for (int j = 0; j < gridSize * 3; j++) {
                    if (expandedGrid[i][j] == 0) {
                        floodFill(expandedGrid, i, j);
                        regionCount++;
                    }
                }
            }
            return regionCount;
        }

        void floodFill(int[][] expandedGrid, int row, int col) {
            Queue<int[]> queue = new LinkedList<>();
            expandedGrid[row][col] = 1;
            queue.add(new int[] { row, col });

            while (!queue.isEmpty()) {
                int[] currentCell = queue.poll();
                for (int[] direction : DIRECTIONS) {
                    int newRow = direction[0] + currentCell[0];
                    int newCol = direction[1] + currentCell[1];

                    if (isValidCell(expandedGrid, newRow, newCol)) {
                        expandedGrid[newRow][newCol] = 1;
                        queue.add(new int[] { newRow, newCol });
                    }
                }
            }
        }

        boolean isValidCell(int[][] expandedGrid, int row, int col) {
            int n = expandedGrid.length;
            return (row >= 0 &&
                    col >= 0 &&
                    row < n &&
                    col < n &&
                    expandedGrid[row][col] == 0);
        }
    }

    static class Solution2 {
        public int regionsBySlashes(String[] grid) {
            int gridSize = grid.length;
            int totalTriangles = gridSize * gridSize * 4;
            int[] parentArray = new int[totalTriangles];
            Arrays.fill(parentArray, -1);

            int regionCount = totalTriangles;

            for (int row = 0; row < gridSize; row++) {
                for (int col = 0; col < gridSize; col++) {
                    if (row > 0) {
                        regionCount -= unionTriangles(
                                parentArray,
                                getTriangleIndex(gridSize, row - 1, col, 2),
                                getTriangleIndex(gridSize, row, col, 0));
                    }
                    if (col > 0) {
                        regionCount -= unionTriangles(
                                parentArray,
                                getTriangleIndex(gridSize, row, col - 1, 1),
                                getTriangleIndex(gridSize, row, col, 3));
                    }

                    if (grid[row].charAt(col) != '/') {
                        regionCount -= unionTriangles(
                                parentArray,
                                getTriangleIndex(gridSize, row, col, 0),
                                getTriangleIndex(gridSize, row, col, 1));
                        regionCount -= unionTriangles(
                                parentArray,
                                getTriangleIndex(gridSize, row, col, 2),
                                getTriangleIndex(gridSize, row, col, 3));
                    }

                    if (grid[row].charAt(col) != '\\') {
                        regionCount -= unionTriangles(
                                parentArray,
                                getTriangleIndex(gridSize, row, col, 0),
                                getTriangleIndex(gridSize, row, col, 3));
                        regionCount -= unionTriangles(
                                parentArray,
                                getTriangleIndex(gridSize, row, col, 2),
                                getTriangleIndex(gridSize, row, col, 1));
                    }
                }
            }
            return regionCount;
        }

        int getTriangleIndex(int gridSize, int row, int col, int triangleNum) {
            return (gridSize * row + col) * 4 + triangleNum;
        }

        int unionTriangles(int[] parentArray, int x, int y) {
            int parentX = findParent(parentArray, x);
            int parentY = findParent(parentArray, y);

            if (parentX != parentY) {
                parentArray[parentX] = parentY;
                return 1;
            }
            return 0;
        }

        int findParent(int[] parentArray, int x) {
            if (parentArray[x] == -1)
                return x;
            return parentArray[x] = findParent(parentArray, parentArray[x]);
        }
    }
}
