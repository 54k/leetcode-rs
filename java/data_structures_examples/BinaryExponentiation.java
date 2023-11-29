package data_structures_examples;

public class BinaryExponentiation {
    // https://leetcode.com/problems/powx-n/description/
    static class Solution1 {
        public double myPow(double x, int n) {
            if (n < 0) {
                n = -n;
                x = 1.0 / x;
            }
            double result = 1.0;
            while (n != 0) {
                if ((n & 1) == 1) {
                    result *= x;
                }
                x *= x;
                n /= 2;
            }
            return result;
        }
    }

    // https://leetcode.com/problems/super-pow/description/
    // https://leetcode.com/problems/super-pow/solutions/4160018/easy-solution-with-explanation/
    // https://leetcode.com/problems/super-pow/solutions/84472/c-clean-and-short-solution/
    static class Solution2 {
        public int superPow(int a, int[] b) {
            int mod = 1337;
            var powMod = new Object() {
                int apply(int a, int k) {
                    a %= mod;
                    int result = 1;
                    while (k > 0) {
                        if (k % 2 == 1) {
                            result = (result * a) % mod;
                            k--;
                        }
                        a = (a % mod * a % mod) % mod;
                        k /= 2;
                    }
                    return result;
                }
            };

            var powRec = new Object() {
                int apply(int a, int bl, int br) {
                    if (br - bl == 0) {
                        return 1;
                    }
                    int lastDigit = b[br - 1];
                    return powMod.apply(apply(a, bl, br - 1), 10) * powMod.apply(a, lastDigit) % mod;
                }
            };

            return powRec.apply(a, 0, b.length);
        }
    }
}
