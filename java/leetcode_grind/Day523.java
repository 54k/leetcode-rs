package leetcode_grind;

public class Day523 {
    // https://leetcode.com/problems/island-perimeter/description
    static class Solution1 {
        public int islandPerimeter(int[][] grid) {
            int m = grid.length, n = grid[0].length, ans = 0;
            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    if (grid[i][j] == 0) {
                        continue;
                    }
                    int p = 4;
                    if (i < m - 1) {
                        p -= grid[i + 1][j] * 2;
                    }
                    if (j > 0) {
                        p -= grid[i][j - 1] * 2;
                    }
                    ans += p;
                }
            }
            return ans;
        }
    }
}
