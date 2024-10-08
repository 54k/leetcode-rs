package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
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

    static class Solution2 {
        private final int[][] directions = {
                { 0, 1 },
                { 1, 0 },
                { 0, -1 },
                { -1, 0 },
        };

        boolean isCellLand(int x, int y, int[][] grid) {
            return grid[x][y] == 1;
        }

        boolean isSubIsland(
                int x,
                int y,
                int[][] grid1,
                int[][] grid2,
                boolean[][] visited) {
            int totalRows = grid2.length;
            int totalCols = grid2[0].length;
            boolean isSubIsland = true;

            if (!isCellLand(x, y, grid1)) {
                isSubIsland = false;
            }

            for (int[] direction : directions) {
                int nextX = x + direction[0];
                int nextY = y + direction[1];

                if (nextX >= 0 && nextY >= 0 && nextX < totalRows && nextY < totalCols &&
                        !visited[nextX][nextY] && isCellLand(nextX, nextY, grid2)) {
                    visited[nextX][nextY] = true;
                    boolean nextCellIsPartOfSubIsland = isSubIsland(
                            nextX,
                            nextY,
                            grid1,
                            grid2,
                            visited);
                    isSubIsland = isSubIsland && nextCellIsPartOfSubIsland;
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
                    if (!visited[x][y] && isCellLand(x, y, grid2)) {
                        visited[x][y] = true;
                        if (isSubIsland(x, y, grid1, grid2, visited)) {
                            subIslandCounts += 1;
                        }
                    }
                }
            }
            return subIslandCounts;
        }
    }

    static class Solution3 {
        final int[][] directions = {
                { 0, 1 },
                { 1, 0 },
                { 0, -1 },
                { -1, 0 },
        };

        boolean isCellLand(int x, int y, int[][] grid) {
            return grid[x][y] == 1;
        }

        static class UnionFind {
            final int[] parent;
            final int[] rank;

            UnionFind(int n) {
                parent = new int[n];
                rank = new int[n];
                for (int i = 0; i < n; i++) {
                    parent[i] = i;
                    rank[i] = 0;
                }
            }

            int find(int u) {
                if (parent[u] != u) {
                    parent[u] = find(parent[u]);
                }
                return parent[u];
            }

            void unionSets(int u, int v) {
                int rootU = find(u);
                int rootV = find(v);
                if (rootU != rootV) {
                    if (rank[rootU] > rank[rootV]) {
                        parent[rootV] = rootU;
                    } else if (rank[rootU] < rank[rootV]) {
                        parent[rootU] = rootV;
                    } else {
                        parent[rootV] = rootU;
                        rank[rootU]++;
                    }
                }
            }
        }

        int convertToIndex(int x, int y, int totalCols) {
            return x * totalCols + y;
        }

        public int countSubIslands(int[][] grid1, int[][] grid2) {
            int totalRows = grid2.length;
            int totalCols = grid2[0].length;
            UnionFind uf = new UnionFind(totalRows * totalCols);

            for (int x = 0; x < totalRows; ++x) {
                for (int y = 0; y < totalCols; ++y) {
                    if (isCellLand(x, y, grid2)) {
                        for (int[] direction : directions) {
                            int nextX = x + direction[0];
                            int nextY = y + direction[1];
                            if (nextX >= 0 &&
                                    nextY >= 0 &&
                                    nextX < totalRows &&
                                    nextY < totalCols &&
                                    isCellLand(nextX, nextY, grid2)) {
                                uf.unionSets(
                                        convertToIndex(x, y, totalCols),
                                        convertToIndex(nextX, nextY, totalCols));
                            }
                        }
                    }
                }
            }

            boolean[] isSubIsland = new boolean[totalRows * totalCols];
            for (int i = 0; i < isSubIsland.length; i++) {
                isSubIsland[i] = true;
            }

            for (int x = 0; x < totalRows; ++x) {
                for (int y = 0; y < totalCols; ++y) {
                    if (isCellLand(x, y, grid2) && !isCellLand(x, y, grid1)) {
                        int root = uf.find(convertToIndex(x, y, totalCols));
                        isSubIsland[root] = false;
                    }
                }
            }

            int subIslandCounts = 0;
            for (int x = 0; x < totalRows; ++x) {
                for (int y = 0; y < totalCols; ++y) {
                    if (isCellLand(x, y, grid2)) {
                        int root = uf.find(convertToIndex(x, y, totalCols));
                        if (isSubIsland[root]) {
                            subIslandCounts++;
                            isSubIsland[root] = false;
                        }
                    }
                }
            }

            return subIslandCounts;
        }
    }

    // https://leetcode.com/problems/number-of-distinct-islands/description/
    static class Solution4 {
        List<List<int[]>> uniqueIslands = new ArrayList<>();
        List<int[]> currentIsland = new ArrayList<>();
        int[][] grid;
        boolean[][] seen;

        public int numDistinctIslands(int[][] grid) {
            this.grid = grid;
            this.seen = new boolean[grid.length][grid[0].length];
            for (int row = 0; row < grid.length; row++) {
                for (int col = 0; col < grid[0].length; col++) {
                    dfs(row, col);
                    if (currentIsland.isEmpty()) {
                        continue;
                    }
                    int minCol = grid[0].length - 1;
                    for (int i = 0; i < currentIsland.size(); i++) {
                        minCol = Math.min(minCol, currentIsland.get(i)[1]);
                    }
                    for (int[] cell : currentIsland) {
                        cell[0] -= row;
                        cell[1] -= minCol;
                    }
                    if (currentIslandUnique()) {
                        uniqueIslands.add(currentIsland);
                    }
                    currentIsland = new ArrayList<>();
                }
            }
            return uniqueIslands.size();
        }

        void dfs(int row, int col) {
            if (row < 0 || col < 0 || row >= grid.length || col >= grid[0].length)
                return;
            if (seen[row][col] || grid[row][col] == 0)
                return;
            seen[row][col] = true;
            currentIsland.add(new int[] { row, col });
            dfs(row + 1, col);
            dfs(row - 1, col);
            dfs(row, col + 1);
            dfs(row, col - 1);
        }

        boolean currentIslandUnique() {
            for (List<int[]> otherIsland : uniqueIslands) {
                if (currentIsland.size() != otherIsland.size()) {
                    continue;
                }
                if (equalIslands(currentIsland, otherIsland)) {
                    return false;
                }
            }
            return true;
        }

        boolean equalIslands(List<int[]> island1, List<int[]> island2) {
            for (int i = 0; i < island1.size(); i++) {
                if (island1.get(i)[0] != island2.get(i)[0] || island1.get(i)[1] != island2.get(i)[1]) {
                    return false;
                }
            }
            return true;
        }
    }
}
