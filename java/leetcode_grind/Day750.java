package leetcode_grind;

public class Day750 {
    // https://leetcode.com/problems/cutting-ribbons/description/
    static class Solution1 {
        public int maxLength(int[] ribbons, int k) {
            int hi = 0, lo = 0;
            for (var r : ribbons) {
                hi = Math.max(r, hi);
            }
            while (lo < hi) {
                int mid = (lo + hi + 1) / 2;
                if (check(ribbons, k, mid)) {
                    lo = mid;
                } else {
                    hi = mid - 1;
                }
            }
            return lo;
        }

        boolean check(int[] ribbons, int k, int len) {
            var ans = 0;
            for (var r : ribbons) {
                ans += r / len;
            }
            return ans >= k;
        }
    }

}
