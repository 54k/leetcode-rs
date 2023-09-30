package leetcode_grind;

import java.util.Arrays;

public class Day322 {
    static class Solution {
        public boolean find132pattern(int[] nums) {
            if (nums.length < 3) {
                return false;
            }
            var min = new int[nums.length];
            min[0] = nums[0];
            for (var i = 1; i < nums.length; i++) {
                min[i] = Math.min(min[i - 1], nums[i]);
            }

            for (int j = nums.length - 1, k = nums.length; j >= 0; j--) {
                if (nums[j] > min[j]) {
                    k = Arrays.binarySearch(nums, k, nums.length, min[j] + 1);
                    if (k < 0) {
                        k = -1 - k;
                    }
                    if (k < nums.length && nums[k] < nums[j]) {
                        return true;
                    }
                    nums[--k] = nums[j];
                }
            }

            return false;
        }
    }
}
