package data_structures_examples;

public class Prefix2D {
    // https://leetcode.com/problems/range-sum-query-2d-immutable/description/
    static class NumMatrix {
        int[][] region;
        int m;
        int n;

        public NumMatrix(int[][] matrix) {
            m = matrix.length;
            n = matrix[0].length;
            region = new int[m + 1][n + 1];

            for (var r = 1; r <= m; r++) {
                for (var c = 1; c <= n; c++) {
                    region[r][c] = matrix[r - 1][c - 1] + region[r][c - 1] + region[r - 1][c] - region[r - 1][c - 1];
                }
            }
        }

        public int sumRegion(int row1, int col1, int row2, int col2) {
            return region[row2 + 1][col2 + 1] - region[row2 + 1][col1] - region[row1][col2 + 1] + region[row1][col1];
        }
    }
}
