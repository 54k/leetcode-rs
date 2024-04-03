package leetcode_grind;

public class Day507 {
    // https://leetcode.com/problems/word-search/description/
    static class Solution1 {
        public boolean exist(char[][] board, String word) {
            int m = board.length;
            int n = board[0].length;

            var rec = new Object() {
                boolean apply(int r, int c, int i) {
                    if (i >= word.length()) {
                        return true;
                    }

                    if (r < 0 || r >= m || c < 0 || c >= n || board[r][c] != word.charAt(i)) {
                        return false;
                    }

                    int[][] dir = { { 0, 1 }, { 0, -1 }, { -1, 0 }, { 1, 0 } };
                    boolean res = false;

                    for (var d : dir) {
                        board[r][c] = '#';
                        if (apply(r + d[0], c + d[1], i + 1)) {
                            return true;
                        }
                        board[r][c] = word.charAt(i);
                    }

                    return res;
                }
            };

            for (int r = 0; r < m; r++) {
                for (int c = 0; c < n; c++) {
                    if (rec.apply(r, c, 0)) {
                        return true;
                    }
                }
            }

            return false;
        }
    }
}
