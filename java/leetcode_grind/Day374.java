package leetcode_grind;

import java.util.HashMap;
import java.util.function.Function;

public class Day374 {
    // https://leetcode.com/problems/count-nice-pairs-in-an-array/
    class Solution1 {
        public int countNicePairs(int[] nums) {
            var mod = 1_000_000_007;
            Function<Integer, Integer> rev = (n) -> {
                int r = 0;
                while (n > 0) {
                    r = r * 10 + n % 10;
                    n /= 10;
                }
                return r;
            };

            var ans = 0;
            var counter = new HashMap<Integer, Integer>();
            for (var num : nums) {
                var sum = num - rev.apply(num);
                ans = (ans + counter.getOrDefault(sum, 0)) % mod;
                counter.put(sum, counter.getOrDefault(sum, 0) + 1);
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/count-number-of-bad-pairs/
    static class Solution2 {
        public long countBadPairs(int[] nums) {
            var counter = new HashMap<Integer, Integer>();
            var n = (long) nums.length;
            long ans = n * (n - 1) / 2;
            for (int i = 0; i < nums.length; i++) {
                var sum = nums[i] - i;
                ans -= counter.getOrDefault(sum, 0);
                counter.put(sum, counter.getOrDefault(sum, 0) + 1);
            }
            return ans;
        }
    }
}
