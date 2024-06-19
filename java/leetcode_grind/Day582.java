package leetcode_grind;

public class Day582 {
    // https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/description
    static class Solution1 {
        public int minDays(int[] bloomDay, int m, int k) {
            var check = new Object() {
                boolean apply(int d) {
                    var b = 0;
                    outer: for (int i = 0; i < bloomDay.length; i++) {
                        for (int j = 0; j < k; j++) {
                            if (i + j >= bloomDay.length || bloomDay[i + j] > d) {
                                continue outer;
                            }
                        }
                        b++;
                        if (b == m) {
                            return true;
                        }
                        i += k - 1;
                    }
                    return b >= m;
                }
            };

            int n = bloomDay.length;
            if (n / k < m) {
                return -1;
            }

            int lo = -1;
            int hi = (int) 1e9;

            while (lo + 1 < hi) {
                int mid = lo + (hi - lo) / 2;
                if (check.apply(mid)) {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            return hi == 0 ? -1 : hi;
        }
    }
}
