package leetcode_grind;

public class Day737 {
    // https://leetcode.com/problems/transpose-matrix/description/
    static class Solution1 {
        public int[][] transpose(int[][] matrix) {
            int R = matrix.length, C = matrix[0].length;
            int[][] ans = new int[C][R];
            for (int r = 0; r < R; ++r) {
                for (int c = 0; c < C; ++c) {
                    ans[c][r] = matrix[r][c];
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/rotating-the-box/description/?envType=daily-question&envId=2024-11-23
    static class Solution2 {
        public char[][] rotateTheBox(char[][] box) {
            int m = box.length;
            int n = box[0].length;
            char[][] result = new char[n][m];

            for (int i = 0; i < n; i++) {
                for (int j = 0; j < m; j++) {
                    result[i][j] = box[j][i];
                }
            }

            for (int i = 0; i < n; i++) {
                reverse(result[i]);
            }

            for (int j = 0; j < m; j++) {
                for (int i = n - 1; i >= 0; i--) {
                    if (result[i][j] == '.') {
                        int nextRowWithStone = -1;
                        for (int k = i - 1; k >= 0; k--) {
                            if (result[k][j] == '*')
                                break;
                            if (result[k][j] == '#') {
                                nextRowWithStone = k;
                                break;
                            }
                        }

                        if (nextRowWithStone != -1) {
                            result[nextRowWithStone][j] = '.';
                            result[i][j] = '#';
                        }
                    }
                }
            }
            return result;
        }

        void reverse(char[] row) {
            int left = 0;
            int right = row.length - 1;
            while (left < right) {
                char temp = row[left];
                row[left] = row[right];
                row[right] = temp;
                left++;
                right--;
            }
        }
    }
}
