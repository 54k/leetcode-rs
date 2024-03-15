package leetcode_grind;

public class Day488 {
    // https://leetcode.com/problems/construct-product-matrix/description/
    static class Solution1 {
        public int[][] constructProductMatrix(int[][] grid) {
            var m = grid.length;
            var n = grid[0].length;
            var MOD = 12345L;

            var product = 1L;
            var productRight = new long[m][n];

            for (int i = m - 1; i >= 0; i--) {
                for (int j = n - 1; j >= 0; j--) {
                    productRight[i][j] = product;
                    product = (product % MOD * grid[i][j] % MOD) % MOD;
                }
            }
            var result = new int[m][n];
            var productLeft = 1;
            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    result[i][j] = (int) ((productLeft % MOD * productRight[i][j] % MOD) % MOD);
                    productLeft = (int) ((productLeft % MOD * grid[i][j] % MOD) % MOD);
                }
            }
            return result;
        }
    }

}
