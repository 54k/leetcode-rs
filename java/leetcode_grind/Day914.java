package leetcode_grind;

public class Day914 {
    // https://leetcode.com/problems/set-matrix-zeroes/description/?envType=daily-question&envId=2025-05-21
    static class Solution1 {
        public void setZeroes(int[][] matrix) {
            int m = matrix.length;
            int n = matrix[0].length;
            boolean[] rows = new boolean[m];
            boolean[] cols = new boolean[n];

            for (int r = 0; r < m; r++) {
                for (int c = 0; c < n; c++) {
                    if (matrix[r][c] == 0) {
                        cols[c] = true;
                        rows[r] = true;
                    }
                }
            }
            for (int r = 0; r < m; r++) {
                for (int c = 0; c < n; c++) {
                    if (rows[r] || cols[c]) {
                        matrix[r][c] = 0;
                    }
                }
            }
        }
    }
}
