package leetcode_grind;

import java.util.Arrays;

public class Day633 {
    // https://leetcode.com/problems/minimum-number-of-days-to-disconnect-island/description/
    static class Solution1 {
        static final int[][] DIRECTIONS = {
                { 0, 1 },
                { 0, -1 },
                { 1, 0 },
                { -1, 0 },
        };

        public int minDays(int[][] grid) {
            int rows = grid.length;
            int cols = grid[0].length;

            int initialIslandCount = countIslands(grid);

            if (initialIslandCount != 1) {
                return 0;
            }

            for (int row = 0; row < rows; row++) {
                for (int col = 0; col < cols; col++) {
                    if (grid[row][col] == 0)
                        continue;

                    grid[row][col] = 0;
                    int newIslandCount = countIslands(grid);

                    if (newIslandCount != 1)
                        return 1;

                    grid[row][col] = 1;
                }
            }

            return 2;
        }

        int countIslands(int[][] grid) {
            int rows = grid.length;
            int cols = grid[0].length;
            boolean[][] visited = new boolean[rows][cols];
            int islandCount = 0;

            for (int row = 0; row < rows; row++) {
                for (int col = 0; col < cols; col++) {
                    if (!visited[row][col] && grid[row][col] == 1) {
                        exploreIsland(grid, row, col, visited);
                        islandCount++;
                    }
                }
            }
            return islandCount;
        }

        void exploreIsland(int[][] grid, int row, int col, boolean[][] visited) {
            visited[row][col] = true;

            for (int[] direction : DIRECTIONS) {
                int newRow = row + direction[0];
                int newCol = col + direction[1];
                if (isValidLandCell(grid, newRow, newCol, visited)) {
                    exploreIsland(grid, newRow, newCol, visited);
                }
            }
        }

        boolean isValidLandCell(int[][] grid, int row, int col, boolean[][] visited) {
            int rows = grid.length;
            int cols = grid[0].length;
            return (row >= 0 &&
                    col >= 0 &&
                    row < rows &&
                    col < cols &&
                    grid[row][col] == 1 &&
                    !visited[row][col]);
        }
    }

    static class Solution2 {
        static final int[][] DIRECTIONS = {
                { 0, 1 },
                { 0, -1 },
                { 1, 0 },
                { -1, 0 },
        };

        public int minDays(int[][] grid) {
            int rows = grid.length;
            int cols = grid[0].length;
            ArticulationPointInfo apInfo = new ArticulationPointInfo(false, 0);
            int landCells = 0, islandCount = 0;

            int[][] discoveryTime = new int[rows][cols];
            int[][] lowestReachable = new int[rows][cols];
            int[][] parentCell = new int[rows][cols];

            for (int i = 0; i < rows; i++) {
                Arrays.fill(discoveryTime[i], -1);
                Arrays.fill(lowestReachable[i], -1);
                Arrays.fill(parentCell[i], -1);
            }

            for (int i = 0; i < rows; i++) {
                for (int j = 0; j < cols; j++) {
                    if (grid[i][j] == 1) {
                        landCells++;
                        if (discoveryTime[i][j] == -1) {
                            findArticulationPoints(
                                    grid,
                                    i,
                                    j,
                                    discoveryTime,
                                    lowestReachable,
                                    parentCell,
                                    apInfo);
                            islandCount++;
                        }
                    }
                }
            }

            if (islandCount == 0 || islandCount >= 2)
                return 0;
            if (landCells == 1)
                return 1;
            if (apInfo.hasArticulationPoint)
                return 1;
            return 2;
        }

        void findArticulationPoints(
                int[][] grid,
                int row,
                int col,
                int[][] discoveryTime,
                int[][] lowestReachable,
                int[][] parentCell,
                ArticulationPointInfo apInfo) {
            int rows = grid.length, cols = grid[0].length;
            discoveryTime[row][col] = apInfo.time;
            apInfo.time++;
            lowestReachable[row][col] = discoveryTime[row][col];
            int children = 0;

            for (int[] direction : DIRECTIONS) {
                int newRow = row + direction[0];
                int newCol = col + direction[1];
                if (isValidLandCell(grid, newRow, newCol)) {
                    if (discoveryTime[newRow][newCol] == -1) {
                        children++;
                        parentCell[newRow][newCol] = row * cols + col;
                        findArticulationPoints(
                                grid,
                                newRow,
                                newCol,
                                discoveryTime,
                                lowestReachable,
                                parentCell,
                                apInfo);

                        lowestReachable[row][col] = Math.min(
                                lowestReachable[row][col],
                                lowestReachable[newRow][newCol]);

                        if (lowestReachable[newRow][newCol] >= discoveryTime[row][col] &&
                                parentCell[row][col] != -1) {
                            apInfo.hasArticulationPoint = true;
                        }
                    } else if (newRow * cols + newCol != parentCell[row][col]) {
                        lowestReachable[row][col] = Math.min(
                                lowestReachable[row][col],
                                discoveryTime[newRow][newCol]);
                    }
                }
            }

            if (parentCell[row][col] == -1 && children > 1) {
                apInfo.hasArticulationPoint = true;
            }
        }

        boolean isValidLandCell(int[][] grid, int row, int col) {
            int rows = grid.length;
            int cols = grid[0].length;
            return (row >= 0 &&
                    col >= 0 &&
                    row < rows &&
                    col < cols &&
                    grid[row][col] == 1);
        }

        static class ArticulationPointInfo {
            boolean hasArticulationPoint;
            int time;

            ArticulationPointInfo(boolean hasArticulationPoint, int time) {
                this.hasArticulationPoint = hasArticulationPoint;
                this.time = time;
            }
        }
    }

    // https://leetcode.com/problems/find-the-count-of-monotonic-pairs-i/description/
    static class Solution3 {

        static final int MOD = 1000000007;
        int[][][] dp;

        int solve(int i, int n, int[] v, int prev1, int prev2) {
            if (i == n) {
                return 1;
            }
            if (dp[i][prev1][prev2] != -1) {
                return dp[i][prev1][prev2];
            }
            int ans = 0;
            for (int j = prev1; j <= v[i]; j++) {
                int x1 = j;
                int x2 = v[i] - j;
                if (prev1 <= x1 && x2 <= prev2) {
                    ans = (ans + solve(i + 1, n, v, x1, x2)) % MOD;
                }
            }
            return dp[i][prev1][prev2] = ans;
        }

        public int countOfPairs(int[] nums) {
            int n = nums.length;
            dp = new int[n][52][52];
            for (int[][] row : dp) {
                for (int[] col : row) {
                    Arrays.fill(col, -1);
                }
            }
            return solve(0, n, nums, 0, 50);
        }
    }

}
