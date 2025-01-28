package leetcode_grind;

import java.util.Arrays;
import java.util.LinkedList;
import java.util.Queue;

public class Day801 {
    // https://leetcode.com/problems/delete-columns-to-make-sorted-ii/description/
    static class Solution1 {
        public int minDeletionSize(String[] A) {
            int N = A.length;
            int W = A[0].length();
            int ans = 0;

            String[] cur = new String[N];
            for (int j = 0; j < W; j++) {
                String[] cur2 = Arrays.copyOf(cur, N);
                for (int i = 0; i < N; i++) {
                    cur2[i] += A[i].charAt(j);
                }

                if (isSorted(cur2)) {
                    cur = cur2;
                } else {
                    ans++;
                }
            }
            return ans;
        }

        boolean isSorted(String[] A) {
            for (int i = 0; i < A.length - 1; i++) {
                if (A[i].compareTo(A[i + 1]) > 0) {
                    return false;
                }
            }
            return true;
        }
    }

    // https://leetcode.com/problems/maximum-number-of-fish-in-a-grid/description/?envType=daily-question&envId=2025-01-28
    static class Solution2 {
        int calculateFishes(int[][] grid, boolean[][] visited, int row, int col) {
            if (row < 0 || row >= grid.length || col < 0 || col >= grid[0].length || grid[row][col] == 0
                    || visited[row][col]) {
                return 0;
            }
            visited[row][col] = true;
            return (grid[row][col] +
                    calculateFishes(grid, visited, row, col + 1) +
                    calculateFishes(grid, visited, row, col - 1) +
                    calculateFishes(grid, visited, row + 1, col) +
                    calculateFishes(grid, visited, row - 1, col));
        }

        public int findMaxFish(int[][] grid) {
            int rows = grid.length, cols = grid[0].length;
            int maxFishCount = 0;
            boolean[][] visited = new boolean[rows][cols];
            for (int row = 0; row < rows; row++) {
                for (int col = 0; col < cols; col++) {
                    if (grid[row][col] > 0 && !visited[row][col]) {
                        maxFishCount = Math.max(maxFishCount, calculateFishes(grid, visited, row, col));
                    }
                }
            }
            return maxFishCount;
        }
    }

    static class Solution3 {
        int countFishes(int[][] grid, boolean[][] visited, int row, int col) {
            int numRows = grid.length, numCols = grid[0].length, fishCount = 0;
            Queue<int[]> q = new LinkedList<>();
            q.add(new int[] { row, col });
            visited[row][col] = true;

            int[] rowDirections = { 0, 0, 1, -1 };
            int[] colDirections = { 1, -1, 0, 0 };

            while (!q.isEmpty()) {
                int[] cell = q.poll();
                row = cell[0];
                col = cell[1];
                fishCount += grid[row][col];

                for (int i = 0; i < 4; i++) {
                    int newRow = row + rowDirections[i];
                    int newCol = col + colDirections[i];

                    if (0 <= newRow && newRow < numRows && 0 <= newCol && newCol < numCols && grid[newRow][newCol] != 0
                            && !visited[newRow][newCol]) {
                        q.add(new int[] { newRow, newCol });
                        visited[newRow][newCol] = true;
                    }
                }
            }
            return fishCount;
        }

        public int findMaxFish(int[][] grid) {
            int numRows = grid.length, numCols = grid[0].length, result = 0;
            boolean[][] visited = new boolean[numRows][numCols];

            for (int i = 0; i < numRows; i++) {
                for (int j = 0; j < numCols; j++) {
                    if (grid[i][j] != 0 && !visited[i][j]) {
                        result = Math.max(result, countFishes(grid, visited, i, j));
                    }
                }
            }
            return result;
        }
    }

    static class Solution4 {
        int findParent(int[] parent, int cellIndex) {
            if (parent[cellIndex] == cellIndex) {
                return cellIndex;
            }
            return parent[cellIndex] = findParent(parent, parent[cellIndex]);
        }

        void unionComponents(int[] parent, int[] componentSize, int[] totalFish, int cellIndex1, int cellIndex2) {
            int root1 = findParent(parent, cellIndex1);
            int root2 = findParent(parent, cellIndex2);
            if (root1 != root2) {
                if (componentSize[root1] < componentSize[root2]) {
                    int temp = root1;
                    root1 = root2;
                    root2 = temp;
                }
                parent[root2] = root1;
                componentSize[root1] += componentSize[root2];
                totalFish[root1] += totalFish[root2];
            }
        }

        public int findMaxFish(int[][] grid) {
            int rowCount = grid.length, columnCount = grid[0].length;
            int totalCells = rowCount * columnCount;

            int[] parent = new int[totalCells];
            int[] componentSize = new int[totalCells];
            int[] totalFish = new int[totalCells];

            for (int cellIndex = 0; cellIndex < totalCells; cellIndex++) {
                parent[cellIndex] = cellIndex;
                componentSize[cellIndex] = 1;
            }

            for (int row = 0; row < rowCount; row++) {
                for (int column = 0; column < columnCount; column++) {
                    int cellIndex = row * columnCount + column;
                    totalFish[cellIndex] = grid[row][column];
                }
            }

            int[] deltaRow = { 0, 0, 1, -1 }, deltaColumn = { 1, -1, 0, 0 };
            for (int row = 0; row < rowCount; row++) {
                for (int column = 0; column < columnCount; column++) {
                    if (grid[row][column] > 0) {
                        int cellIndex = row * columnCount + column;
                        for (int direction = 0; direction < 4; direction++) {
                            int neighborRow = row + deltaRow[direction];
                            int neighborColumn = column + deltaColumn[direction];

                            if (neighborRow >= 0 && neighborRow < rowCount && neighborColumn >= 0
                                    && neighborColumn < columnCount && grid[neighborRow][neighborColumn] > 0) {
                                int neighborIndex = neighborRow * columnCount + neighborColumn;
                                unionComponents(parent, componentSize, totalFish, cellIndex, neighborIndex);
                            }
                        }
                    }
                }
            }

            int maxFish = 0;
            for (int cellIndex = 0; cellIndex < totalCells; cellIndex++) {
                if (findParent(parent, cellIndex) == cellIndex) {
                    maxFish = Math.max(maxFish, totalFish[cellIndex]);
                }
            }
            return maxFish;
        }
    }
}
