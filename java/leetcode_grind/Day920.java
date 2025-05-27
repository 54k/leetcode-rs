package leetcode_grind;

public class Day920 {
    // https://leetcode.com/problems/divisible-and-non-divisible-sums-difference/description/
    static class Solution1 {
        public int differenceOfSums(int n, int m) {
            int ans = 0;
            for (int i = 1; i <= n; i++) {
                if (i % m == 0) {
                    ans -= i;
                } else {
                    ans += i;
                }
            }
            return ans;
        }
    }

    static class Solution2 {
        public int differenceOfSums(int n, int m) {
            int k = n / m;
            return (n * (n + 1)) / 2 - k * (k + 1) * m;
        }
    }
}
