package leetcode_grind;

import java.util.ArrayList;

public class Day350 {
    // https://leetcode.com/problems/diagonal-traverse/description/
    static class Solution1 {
        public int[] findDiagonalOrderTraverseAndReverse(int[][] mat) {
            var n = mat.length;
            var m = mat[0].length;

            var result = new int[n * m];
            for (int d = 0, k = 0; d < n + m - 1; d++) {
                var diag = new ArrayList<Integer>();

                var r = d < m ? 0 : d - m + 1;
                var c = d < m ? d : m - 1;

                while (c > -1 && r < n) {
                    diag.add(mat[r][c]);
                    r++;
                    c--;
                }

                for (int i = 0; i < diag.size() / 2 && d % 2 == 0; i++) {
                    var tmp = diag.get(i);
                    diag.set(i, diag.get(diag.size() - i - 1));
                    diag.set(diag.size() - i - 1, tmp);
                }

                for (var num : diag)
                    result[k++] = num;
            }

            return result;
        }

        public int[] findDiagonalOrderSimulation(int[][] mat) {
            var n = mat.length;
            var m = mat[0].length;

            var dir = 1;
            var result = new int[n * m];
            var r = 0;

            var row = 0;
            var col = 0;

            while (row < n && col < m) {
                result[r++] = mat[row][col];

                var newRow = row + (dir == 1 ? -1 : 1);
                var newCol = col + (dir == 1 ? 1 : -1);

                if (newRow < 0 || newRow == n || newCol < 0 || newCol == m) {
                    if (dir == 1) {
                        row += col == m - 1 ? 1 : 0;
                        col += col < m - 1 ? 1 : 0;
                    } else {
                        col += row == n - 1 ? 1 : 0;
                        row += row < n - 1 ? 1 : 0;
                    }
                    dir = 1 - dir;
                } else {
                    row = newRow;
                    col = newCol;
                }
            }

            return result;
        }
    }

    // https://leetcode.com/problems/decode-the-slanted-ciphertext/
    static class Solution2 {
        public String decodeCiphertext1(String encodedText, int rows) {
            var cols = encodedText.length() / rows;
            var mat = new char[rows][cols];
            for (int i = 0, c = 0; i < rows; i++) {
                for (int j = 0; j < cols; j++) {
                    var ch = encodedText.charAt(c++);
                    mat[i][j] = ch;
                }
            }

            var result = new StringBuilder();
            var row = 0;
            var col = 0;

            var len = 0;
            var last = 0;
            var colIdx = 0;

            while (col < cols) {
                if (mat[row][col] != ' ') {
                    last = len;
                }
                result.append(mat[row][col]);
                len++;

                var newRow = row + 1;
                var newCol = col + 1;
                if (newRow >= rows || newCol >= cols) {
                    row = 0;
                    col = ++colIdx;
                } else {
                    row = newRow;
                    col = newCol;
                }
            }

            if (len == 0) {
                return result.toString();
            }

            return result.toString().substring(0, last + 1);
        }

        public String decodeCiphertext2(String encodedText, int rows) {
            var n = encodedText.length();
            var col = n / rows;
            var sb = new StringBuilder();
            for (int i = 0; i < col; i++) {
                var j = i;
                while (j < n) {
                    sb.append(encodedText.charAt(j));
                    j += col + 1;
                }
            }
            return sb.toString().stripTrailing();
        }

        public String decodeCiphertext3(String encodedText, int rows) {
            var cols = encodedText.length() / rows;
            var mat = new char[rows][cols];

            for (int i = 0, s = 0; i < rows; i++) {
                for (int j = 0; j < cols; j++) {
                    mat[i][j] = encodedText.charAt(s++);
                }
            }

            var sb = new StringBuilder();
            for (int i = 0; i < cols; i++) {
                var x = 0;
                var y = i;
                while (x < rows && y < cols) {
                    sb.append(mat[x][y]);
                    x++;
                    y++;
                }
            }

            return sb.toString().stripTrailing();
        }
    }
}
