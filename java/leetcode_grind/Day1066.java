package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day1066 {
    // https://leetcode.com/problems/final-value-of-variable-after-performing-operations/description/?envType=daily-question&envId=2025-10-20
    static class Solution1 {
        public int finalValueAfterOperations(String[] operations) {
            int ret = 0;
            for (String op : operations) {
                if (op.contains("-")) {
                    ret--;
                } else {
                    ret++;
                }
            }
            return ret;
        }
    }

    // https://leetcode.com/problems/bulls-and-cows/description/
    static class Solution2 {
        public String getHint(String secret, String guess) {
            Map<Character, Integer> h = new HashMap<>();
            for (char s : secret.toCharArray()) {
                h.put(s, h.getOrDefault(s, 0) + 1);
            }

            int bulls = 0, cows = 0;
            int n = guess.length();
            for (int idx = 0; idx < n; ++idx) {
                char ch = guess.charAt(idx);
                if (h.containsKey(ch)) {
                    if (ch == secret.charAt(idx)) {
                        bulls++;
                        if (h.get(ch) <= 0) {
                            cows--;
                        }
                    } else {
                        if (h.get(ch) > 0) {
                            cows++;
                        }
                    }
                    h.put(ch, h.get(ch) - 1);
                }
            }
            return Integer.toString(bulls) + "A" + Integer.toString(cows) + "B";
        }
    }

    static class Solution3 {
        public String getHint(String secret, String guess) {
            Map<Character, Integer> h = new HashMap<>();

            int bulls = 0, cows = 0;
            int n = guess.length();
            for (int idx = 0; idx < n; ++idx) {
                char s = secret.charAt(idx);
                char g = guess.charAt(idx);
                if (s == g) {
                    bulls++;
                } else {
                    if (h.getOrDefault(s, 0) < 0) {
                        cows++;
                    }
                    if (h.getOrDefault(g, 0) > 0) {
                        cows++;
                    }
                    h.put(s, h.getOrDefault(s, 0) + 1);
                    h.put(g, h.getOrDefault(g, 0) - 1);
                }
            }
            return Integer.toString(bulls) + "A" + Integer.toString(cows) + "B";
        }
    }

    // https://leetcode.com/problems/range-sum-query-2d-immutable/description/
    class NumMatrix {
        int[][] dp;

        public NumMatrix(int[][] matrix) {
            if (matrix.length == 0 || matrix[0].length == 0)
                return;
            dp = new int[matrix.length + 1][matrix[0].length + 1];
            for (int r = 0; r < matrix.length; r++) {
                for (int c = 0; c < matrix[0].length; c++) {
                    dp[r + 1][c + 1] = dp[r + 1][c] + dp[r][c + 1] + matrix[r][c] - dp[r][c];
                }
            }
        }

        public int sumRegion(int row1, int col1, int row2, int col2) {
            return dp[row2 + 1][col2 + 1] - dp[row1][col2 + 1] - dp[row2 + 1][col1] + dp[row1][col1];
        }
    }

    /**
     * Your NumMatrix object will be instantiated and called as such:
     * NumMatrix obj = new NumMatrix(matrix);
     * int param_1 = obj.sumRegion(row1,col1,row2,col2);
     */
}
