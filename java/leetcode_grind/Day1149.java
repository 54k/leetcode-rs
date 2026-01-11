package leetcode_grind;

public class Day1149 {
    // https://leetcode.com/problems/maximal-rectangle/description/?envType=daily-question&envId=2026-01-11
    static class Solution1 {
        public int maximalRectangle(char[][] matrix) {
            if (matrix.length == 0)
                return 0;
            int maxarea = 0;
            int[][] dp = new int[matrix.length][matrix[0].length];

            for (int i = 0; i < matrix.length; i++) {
                for (int j = 0; j < matrix[0].length; j++) {
                    if (matrix[i][j] == '1') {
                        dp[i][j] = j == 0 ? 1 : dp[i][j - 1] + 1;

                        int width = dp[i][j];

                        for (int k = i; k >= 0; k--) {
                            width = Math.min(width, dp[k][j]);
                            maxarea = Math.max(maxarea, width * (i - k + 1));
                        }
                    }
                }
            }

            return maxarea;
        }
    }

}
