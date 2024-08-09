package leetcode_grind;

public class Day631 {

    // https://leetcode.com/problems/magic-squares-in-grid/description/?envType=daily-question&envId=2024-08-09
    static class Solution1 {
        public int numMagicSquaresInside(int[][] grid) {
            int ans = 0;
            int m = grid.length;
            int n = grid[0].length;

            for (int i = 0; i + 2 < m; i++) {
                for (int j = 0; j + 2 < n; j++) {
                    if (isMagicSquare(grid, i, j)) {
                        ans++;
                    }
                }
            }

            return ans;
        }

        boolean isMagicSquare(int[][] grid, int row, int col) {
            boolean[] seen = new boolean[10];
            for (int i = 0; i < 3; i++) {
                for (int j = 0; j < 3; j++) {
                    int num = grid[row + i][col + j];
                    if (num < 1 || num > 9)
                        return false;
                    if (seen[num])
                        return false;
                    seen[num] = true;
                }
            }

            int diagonal1 = grid[row][col] + grid[row + 1][col + 1] + grid[row + 2][col + 2];
            int diagonal2 = grid[row + 2][col] + grid[row + 1][col + 1] + grid[row][col + 2];

            if (diagonal1 != diagonal2)
                return false;

            // check if all row sums share the same value as the diagonal sums
            int row1 = grid[row][col] + grid[row][col + 1] + grid[row][col + 2];
            int row2 = grid[row + 1][col] +
                    grid[row + 1][col + 1] +
                    grid[row + 1][col + 2];
            int row3 = grid[row + 2][col] +
                    grid[row + 2][col + 1] +
                    grid[row + 2][col + 2];

            if (!(row1 == diagonal1 && row2 == diagonal1 && row3 == diagonal1)) {
                return false;
            }

            // check if all column sums share same value as the diagonal sums
            int col1 = grid[row][col] + grid[row + 1][col] + grid[row + 2][col];
            int col2 = grid[row][col + 1] +
                    grid[row + 1][col + 1] +
                    grid[row + 2][col + 1];
            int col3 = grid[row][col + 2] +
                    grid[row + 1][col + 2] +
                    grid[row + 2][col + 2];

            if (!(col1 == diagonal1 && col2 == diagonal1 && col3 == diagonal2)) {
                return false;
            }

            return true;
        }
    }
}
