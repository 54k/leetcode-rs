package leetcode_grind;

public class Day846 {
    // https://leetcode.com/problems/maximum-candies-allocated-to-k-children/description/?envType=daily-question&envId=2025-03-14
    static class Solution1 {
        public int maximumCandies(int[] candies, long k) {
            var check = new Object() {
                boolean apply(int lim) {
                    var left = k;
                    for (var c : candies) {
                        if (c / lim != 0)
                            left -= (c / lim);
                    }
                    return left <= 0;
                }
            };

            int lower = 0;
            int upper = 1_000_000_000;

            while (lower + 1 < upper) {
                int mid = lower + (upper - lower) / 2;
                if (check.apply(mid)) {
                    lower = mid;
                } else {
                    upper = mid;
                }
            }
            return lower;
        }
    }
}
