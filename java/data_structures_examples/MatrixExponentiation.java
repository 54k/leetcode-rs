package data_structures_examples;

public class MatrixExponentiation {
    // https://leetcode.com/problems/knight-dialer/description/
    static class Solution {
        public int knightDialer(int n) {
            if (n == 1) {
                return 10;
            }

            long mod = 1_000_000_007;

            long[][] A = new long[][] {
                    { 0, 0, 0, 0, 1, 0, 1, 0, 0, 0 },
                    { 0, 0, 0, 0, 0, 0, 1, 0, 1, 0 },
                    { 0, 0, 0, 0, 0, 0, 0, 1, 0, 1 },
                    { 0, 0, 0, 0, 1, 0, 0, 0, 1, 0 },
                    { 1, 0, 0, 1, 0, 0, 0, 0, 0, 1 },
                    { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 },
                    { 1, 1, 0, 0, 0, 0, 0, 1, 0, 0 },
                    { 0, 0, 1, 0, 0, 0, 1, 0, 0, 0 },
                    { 0, 1, 0, 1, 0, 0, 0, 0, 0, 0 },
                    { 0, 0, 1, 0, 1, 0, 0, 0, 0, 0 }
            };

            long[][] v = new long[][] {
                    { 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 }
            };

            var multiply = new Object() {
                long[][] apply(long[][] a, long[][] b) {
                    long[][] result = new long[a.length][b[0].length];

                    for (int i = 0; i < a.length; i++) {
                        for (int j = 0; j < b[0].length; j++) {
                            for (int k = 0; k < b.length; k++) {
                                result[i][j] = (result[i][j] + a[i][k] * b[k][j]) % mod;
                            }
                        }
                    }
                    return result;
                }
            };

            n--;
            while (n > 0) {
                if ((n & 1) != 0) {
                    v = multiply.apply(v, A);
                }

                A = multiply.apply(A, A);
                n >>= 1;
            }

            long ans = 0;
            for (long num : v[0]) {
                ans = (ans + num) % mod;
            }

            return (int) ans;
        }
    }
}
