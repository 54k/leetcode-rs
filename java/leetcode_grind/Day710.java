package leetcode_grind;

public class Day710 {
    // https://leetcode.com/problems/count-square-submatrices-with-all-ones/description/?envType=daily-question&envId=2024-10-27
    static class Solution1 {
        public int countSquares(int[][] matrix) {
            var m = matrix.length;
            var n = matrix[0].length;
            var ans = 0;
            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    if (matrix[i][j] > 0 && i > 0 && j > 0) {
                        matrix[i][j] = 1 + Math.min(matrix[i - 1][j], Math.min(matrix[i - 1][j - 1], matrix[i][j - 1]));
                    }
                    ans += matrix[i][j];
                }
            }
            return ans;
        }
    }
}
