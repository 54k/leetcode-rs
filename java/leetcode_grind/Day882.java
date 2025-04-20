package leetcode_grind;

import java.util.Arrays;

public class Day882 {
    // https://leetcode.com/problems/count-the-number-of-fair-pairs/description/
    static class Solution1 {
        public long countFairPairs(int[] nums, int lower, int upper) {
            Arrays.sort(nums);
            return lower_bound(nums, upper + 1) - lower_bound(nums, lower);
        }

        long lower_bound(int[] nums, int value) {
            int left = 0, right = nums.length - 1;
            long result = 0;
            while (left < right) {
                int sum = nums[left] + nums[right];
                if (sum < value) {
                    result += (right - left);
                    left++;
                } else {
                    right--;
                }
            }
            return result;
        }
    }

    static class Solution2 {
        long lower_bound(int[] nums, int low, int high, int element) {
            while (low <= high) {
                int mid = low + ((high - low) / 2);
                if (nums[mid] >= element) {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            }
            return low;
        }

        public long countFairPairs(int[] nums, int lower, int upper) {
            Arrays.sort(nums);
            long ans = 0;
            for (int i = 0; i < nums.length; i++) {
                long low = lower_bound(nums, i + 1, nums.length - 1, lower - nums[i]);
                long high = lower_bound(nums, i + 1, nums.length - 1, upper - nums[i] + 1);
                ans += 1 * (high - low);
            }
            return ans;
        }
    }
}
