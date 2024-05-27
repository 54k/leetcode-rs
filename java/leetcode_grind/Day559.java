package leetcode_grind;

public class Day559 {
    // https://leetcode.com/problems/special-array-with-x-elements-greater-than-or-equal-x/description
    static class Solution1 {
        void sort(int[] nums) {
            var bucket = new int[1001];
            for (var n : nums) {
                bucket[n]++;
            }
            for (int i = 0, j = 0; i < bucket.length; i++) {
                while (bucket[i]-- > 0) {
                    nums[j++] = i;
                }
            }
        }

        public int specialArray(int[] nums) {
            sort(nums);

            for (int val = 1; val <= nums.length; val++) {
                int lo = -1, hi = nums.length;
                while (lo + 1 < hi) {
                    int mid = (lo + hi) / 2;
                    if (nums[mid] >= val) {
                        hi = mid;
                    } else {
                        lo = mid;
                    }
                }
                if (nums.length - hi == val) {
                    return val;
                }
            }
            return -1;
        }
    }
}
