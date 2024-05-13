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
}
