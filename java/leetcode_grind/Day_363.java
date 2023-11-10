package leetcode_grind;

public class Day_363 {
    // https://leetcode.com/problems/maximal-square/description
    static class Solution1 {
        public int maximalSquare1(char[][] matrix) {
            var rows = matrix.length;
            var cols = rows > 0 ? matrix[0].length : 0;
            var maxsqlen = 0;
            var dp = new int[matrix.length + 1][matrix[0].length + 1];
            for (int i = 1; i <= rows; i++) {
                for (int j = 1; j <= cols; j++) {
                    if (matrix[i - 1][j - 1] == '1') {
                        dp[i][j] = 1 + Math.min(dp[i][j - 1], Math.min(dp[i - 1][j], dp[i - 1][j - 1]));
                    }
                    maxsqlen = Math.max(maxsqlen, dp[i][j]);
                }
            }

            return maxsqlen * maxsqlen;
        }

        public int maximalSquare2(char[][] matrix) {
            var rows = matrix.length;
            var cols = rows > 0 ? matrix[0].length : 0;
            var dp = new int[cols + 1];
            var maxsqlen = 0;
            var prev = 0;

            for (int i = 1; i <= rows; i++) {
                for (int j = 1; j <= cols; j++) {
                    var temp = dp[j];
                    if (matrix[i - 1][j - 1] == '1') {
                        dp[j] = Math.min(Math.min(dp[j - 1], prev), dp[j]) + 1;
                        maxsqlen = Math.max(maxsqlen, dp[j]);
                    } else {
                        dp[j] = 0;
                    }
                    prev = temp;
                }
            }
            return maxsqlen * maxsqlen;
        }
    }
}
