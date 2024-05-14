package leetcode_grind;

public class Day547 {
    static class Solution1 {
        // https://leetcode.com/problems/path-with-maximum-gold/description
        public int getMaximumGold(int[][] grid) {
            var m = grid.length;
            var n = grid[0].length;

            var dfs = new Object() {
                int apply(int i, int j) {
                    if (0 > i || i >= m || 0 > j || j >= n || grid[i][j] == 0) {
                        return 0;
                    }

                    var tot = grid[i][j];
                    grid[i][j] = 0;
                    var nc = 0;
                    for (var d : new int[][] { { -1, 0 }, { 0, 1 }, { 1, 0 }, { 0, -1 } }) {
                        var ni = i + d[0];
                        var nj = j + d[1];
                        nc = Math.max(nc, apply(ni, nj));
                    }
                    grid[i][j] = tot;
                    tot += nc;
                    return tot;
                }
            };

            var mx = 0;
            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    if (grid[i][j] != 0) {
                        mx = Math.max(dfs.apply(i, j), mx);
                    }
                }
            }

            return mx;
        }
    }
}
