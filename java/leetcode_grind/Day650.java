package leetcode_grind;

import java.util.LinkedList;
import java.util.Queue;

public class Day650 {
    // https://leetcode.com/problems/count-sub-islands/description/?envType=daily-question&envId=2024-08-28
    static class Solution1 {
        private final int[][] directions = {
                { 0, 1 },
                { 1, 0 },
                { 0, -1 },
                { -1, 0 },
        };

        boolean isCellLand(int x, int y, int[][] grid) {
            return grid[x][y] == 1;
        }

        boolean isSubIsland(int x, int y, int[][] grid1, int[][] grid2, boolean[][] visited) {
            int totalRows = grid2.length;
            int totalCols = grid2[0].length;

            boolean isSubIsland = true;

            Queue<int[]> pendingCells = new LinkedList<>();
            pendingCells.offer(new int[] { x, y });
            visited[x][y] = true;

            while (!pendingCells.isEmpty()) {
                int[] curr = pendingCells.poll();
                int currX = curr[0];
                int currY = curr[1];

                if (!isCellLand(currX, currY, grid1)) {
                    isSubIsland = false;
                }

                for (int[] direction : directions) {
                    int nextX = currX + direction[0];
                    int nextY = currY + direction[1];

                    if (nextX >= 0 && nextY >= 0 && nextX < totalRows && nextY < totalCols &&
                            !visited[nextX][nextY] && isCellLand(nextX, nextY, grid2)) {
                        pendingCells.offer(new int[] { nextX, nextY });
                        visited[nextX][nextY] = true;
                    }
                }
            }

            return isSubIsland;
        }

        public int countSubIslands(int[][] grid1, int[][] grid2) {
            int totalRows = grid2.length;
            int totalCols = grid2[0].length;

            boolean[][] visited = new boolean[totalRows][totalCols];
            int subIslandCounts = 0;

            for (int x = 0; x < totalRows; ++x) {
                for (int y = 0; y < totalCols; ++y) {
                    if (!visited[x][y] && isCellLand(x, y, grid2) && isSubIsland(x, y, grid1, grid2, visited)) {
                        subIslandCounts += 1;
                    }
                }
            }

            return subIslandCounts;
        }
    }
}
