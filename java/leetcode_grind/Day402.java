package leetcode_grind;

public class Day402 {
    // https://leetcode.com/problems/image-smoother/description/
    static class Solution {
        public int[][] imageSmoother(int[][] img) {
            int m = img.length;
            int n = img[0].length;

            int[][] smoothing = new int[m][n];

            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    int sum = 0;
                    int count = 0;

                    for (int x = i - 1; x <= i + 1; x++) {
                        for (int y = j - 1; y <= j + 1; y++) {
                            if (0 <= x & x < m && 0 <= y && y < n) {
                                sum += img[x][y];
                                count += 1;
                            }
                        }
                    }

                    smoothing[i][j] = sum / count;
                }
            }

            return smoothing;
        }
    }
}
