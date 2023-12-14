package leetcode_grind;

public class Day397 {
    // https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column/description
    static class Solution1 {
        public int[][] onesMinusZeros(int[][] grid) {
            int m = grid.length;
            int n = grid[0].length;

            int[] rows = new int[m];
            int[] cols = new int[n];

            for (int r = 0; r < m; r++) {
                for (int c = 0; c < n; c++) {
                    rows[r] += grid[r][c];
                    cols[c] += grid[r][c];
                }
            }

            int[][] diff = new int[m][n];

            for (int r = 0; r < m; r++) {
                for (int c = 0; c < n; c++) {
                    diff[r][c] = 2 * (rows[r] + cols[c]) - (m + n);
                }
            }

            return diff;
        }
    }
}
