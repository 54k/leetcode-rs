package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

class Day525 {

    // https://leetcode.com/problems/find-all-groups-of-farmland/description
    static class Solution1 {
        int[][] dirs = { { -1, 0 }, { 1, 0 }, { 0, 1 }, { 0, -1 } };
        int row2, col2;

        private boolean isWithinFarm(int x, int y, int N, int M) {
            return x >= 0 && x < N && y >= 0 && y < M;
        }

        void dfs(int[][] land, boolean[][] visited, int x, int y) {
            visited[x][y] = true;
            row2 = Math.max(row2, x);
            col2 = Math.max(col2, y);

            for (int[] dir : dirs) {
                int newX = x + dir[0], newY = y + dir[1];

                if (isWithinFarm(newX, newY, land.length, land[0].length) && !visited[newX][newY]
                        && land[newX][newY] == 1) {
                    dfs(land, visited, newX, newY);
                }
            }
        }

        void bfs(int[][] land, boolean[][] visited, int sx, int sy) {
            int[] ans = new int[4];
            Queue<int[]> q = new LinkedList<>();

            q.add(new int[] { sx, sy });
            visited[sx][sy] = true;

            while (!q.isEmpty()) {
                int[] v = q.remove();
                int x = v[0], y = v[1];
                row2 = x;
                col2 = y;

                for (int[] dir : dirs) {
                    int newX = x + dir[0], newY = y + dir[1];

                    if (isWithinFarm(newX, newY, land.length, land[0].length) && !visited[newX][newY]
                            && land[newX][newY] == 1) {
                        visited[newX][newY] = true;
                        q.add(new int[] { newX, newY });
                    }
                }
            }
        }

        public int[][] findFarmland(int[][] land) {
            boolean[][] visited = new boolean[land.length][land[0].length];
            List<int[]> ans = new ArrayList<>();

            for (int row1 = 0; row1 < land.length; row1++) {
                for (int col1 = 0; col1 < land[0].length; col1++) {
                    if (land[row1][col1] == 1 && !visited[row1][col1]) {
                        row2 = 0;
                        col2 = 0;

                        bfs(land, visited, row1, col1);

                        int[] arr = new int[] { row1, col1, row2, col2 };
                        ans.add(arr);
                    }
                }
            }

            return ans.stream().toArray(int[][]::new);
        }
    }
}