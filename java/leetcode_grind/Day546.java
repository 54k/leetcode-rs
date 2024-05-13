package leetcode_grind;

public class Day546 {
    static class Solution1 {
        public int matrixScore(int[][] grid) {
            int m = grid.length;
            int n = grid[0].length;
            for (int i = 0; i < m; i++) {
                if (grid[i][0] == 0) {
                    for (int j = 0; j < n; j++) {
                        grid[i][j] = 1 - grid[i][j];
                    }
                }
            }
            for (int j = 1; j < n; j++) {
                int countZero = 0;
                for (int i = 0; i < m; i++) {
                    if (grid[i][j] == 0) {
                        countZero++;
                    }
                }
                if (countZero > m - countZero) {
                    for (int i = 0; i < m; i++) {
                        grid[i][j] = 1 - grid[i][j];
                    }
                }
            }

            int score = 0;
            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    int columnScore = grid[i][j] << (n - j - 1);
                    score += columnScore;
                }
            }
            return score;
        }
    }

    // https://leetcode.com/problems/handshakes-that-dont-cross/description
    static class Solution2 {
        static int m = 1000000007;

        int mul(int a, int b) {
            return (int) ((long) a * b % m);
        }

        public int numberOfWays(int numPeople) {
            int n = numPeople / 2;
            int[] inv = new int[numPeople / 2 + 2];
            inv[1] = 1;
            for (int i = 2; i < n + 2; i++) {
                int k = m / i, r = m % i;
                inv[i] = m - mul(k, inv[r]);
            }
            int C = 1;
            for (int i = 0; i < n; i++) {
                C = mul(mul(2 * (2 * i + 1), inv[i + 2]), C);
            }
            return C;
        }
    }

    // https://leetcode.com/problems/decode-ways-ii/description
    static class Solution3 {
        int M = 1000000007;

        public int numDecodings(String s) {
            Long[] memo = new Long[s.length()];
            return (int) ways(s, s.length() - 1, memo);
        }

        long ways(String s, int i, Long[] memo) {
            if (i < 0) {
                return 1;
            }
            if (memo[i] != null) {
                return memo[i];
            }
            if (s.charAt(i) == '*') {
                long res = 9 * ways(s, i - 1, memo) % M;
                if (i > 0 && s.charAt(i - 1) == '1') {
                    res = (res + 9 * ways(s, i - 2, memo)) % M;
                } else if (i > 0 && s.charAt(i - 1) == '2') {
                    res = (res + 6 * ways(s, i - 2, memo)) % M;
                } else if (i > 0 && s.charAt(i - 1) == '*') {
                    res = (res + 15 * ways(s, i - 2, memo)) % M;
                }
                memo[i] = res;
                return memo[i];
            }
            long res = s.charAt(i) != '0' ? ways(s, i - 1, memo) : 0;
            if (i > 0 && s.charAt(i - 1) == '1') {
                res = (res + ways(s, i - 2, memo)) % M;
            } else if (i > 0 && s.charAt(i - 1) == '2' && s.charAt(i) <= '6') {
                res = (res + ways(s, i - 2, memo)) % M;
            } else if (i > 0 && s.charAt(i - 1) == '*') {
                res = (res + (s.charAt(i) <= '6' ? 2 : 1) * ways(s, i - 2, memo)) % M;
            }
            memo[i] = res;
            return memo[i];
        }
    }
}
