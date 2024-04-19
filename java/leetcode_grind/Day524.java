package leetcode_grind;

public class Day524 {
    // https://leetcode.com/problems/number-of-islands/description
    static class Solution1 {
        public int numIslands(char[][] grid) {
            var m = grid.length;
            var n = grid[0].length;
            var ans = 0;
            var dfs = new Object() {
                void dfs(int[] v) {
                    for (var d : new int[][] { { 0, 1 }, { 1, 0 }, { -1, 0 }, { 0, -1 } }) {
                        var u = new int[] { v[0] + d[0], v[1] + d[1] };
                        if (0 <= u[0] && u[0] < m && 0 <= u[1] && u[1] < n && grid[u[0]][u[1]] == '1') {
                            grid[u[0]][u[1]] = '0';
                            dfs(u);
                        }
                    }
                }
            };
            for (int r = 0; r < m; r++) {
                for (int c = 0; c < n; c++) {
                    if (grid[r][c] == '1') {
                        grid[r][c] = '0';
                        dfs.dfs(new int[] { r, c });
                        ans++;
                    }
                }
            }
            return ans;
        }
    }
}
