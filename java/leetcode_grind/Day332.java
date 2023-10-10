package leetcode_grind;

import java.util.Arrays;
import java.util.function.Function;

public class Day332 {
    // https://leetcode.com/problems/minimum-number-of-operations-to-make-array-continuous/description/
    static class Solution {
        public int minOperations(int[] nums) {
            var n = nums.length;
            var sorted = Arrays.stream(nums).distinct().sorted().toArray();
            var ans = n;

            Function<Integer, Integer> binSearch = (target) -> {
                var lo = 0;
                var hi = sorted.length;
                while (lo < hi) {
                    var mid = (lo + hi) / 2;
                    if (sorted[mid] <= target) {
                        lo = mid + 1;
                    } else {
                        hi = mid;
                    }
                }
                return lo;
            };

            for (int i = 0; i < sorted.length; i++) {
                var left = sorted[i];
                var right = left + n - 1;
                var j = binSearch.apply(right);
                var count = j - i;
                ans = Math.min(ans, n - count);
            }

            return ans;
        }
    }
}
