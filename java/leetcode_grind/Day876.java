package leetcode_grind;

public class Day876 {
    // https://leetcode.com/problems/count-good-numbers/description/?envType=daily-question&envId=2025-04-13
    static class Solution1 {
        long mod = 1000000007;

        public int countGoodNumbers(long n) {
            return (int) ((quickmul(5, (n + 1) / 2) * quickmul(4, n / 2)) % mod);
        }

        public long quickmul(int x, long y) {
            long ret = 1;
            long mul = x;

            while (y > 0) {
                if (y % 2 == 1) {
                    ret = (ret * mul) % mod;
                }
                mul = (mul * mul) % mod;
                y /= 2;
            }

            return ret;
        }
    }
}
