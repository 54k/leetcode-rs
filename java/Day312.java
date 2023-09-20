import java.util.function.Function;
import java.util.function.Supplier;

public class Day312 {
    // https://leetcode.com/problems/find-the-duplicate-number/description/
    static class Solution {
        public int findDuplicateBinarySearch(int[] nums) {
            var left = 1;
            var right = nums.length;
            var duplicate = -1;
            while (left <= right) {
                var mid = (left + right) / 2;

                var count = 0;
                for (var num : nums) {
                    if (num <= mid) {
                        count++;
                    }
                }

                if (count > mid) {
                    duplicate = mid;
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            return duplicate;
        }

        public int findDuplicateCountOfBitSet(int[] nums) {
            Supplier<Integer> maxNum = () -> {
                var ans = 0;
                for (var num : nums) {
                    ans = Math.max(ans, num);
                }
                return ans;
            };
            Function<Integer, Integer> maxBit = (num) -> {
                var ans = 0;
                while (num > 0) {
                    ans++;
                    num >>= 1;
                }
                return ans;
            };

            var duplicate = 0;
            var max = maxBit.apply(maxNum.get());
            for (var bit = 0; bit < max; bit++) {
                var mask = 1 << bit;
                var baseCount = 0;
                var bitCount = 0;

                for (var i = 0; i < nums.length; i++) {
                    if ((mask & i) > 0) {
                        baseCount++;
                    }
                    if ((mask & nums[i]) > 0) {
                        bitCount++;
                    }
                }

                if (bitCount > baseCount) {
                    duplicate |= mask;
                }
            }
            return duplicate;
        }
    }
}
