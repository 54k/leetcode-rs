package leetcode_grind;

import java.util.Arrays;

public class Day937 {
    // https://leetcode.com/problems/minimize-the-maximum-difference-of-pairs/description/?envType=daily-question&envId=2025-06-13
    static class Solution1 {
        int countValidPairs(int[] nums, int threshold) {
            int index = 0, count = 0;
            while (index < nums.length - 1) {
                if (nums[index + 1] - nums[index] <= threshold) {
                    count++;
                    index++;
                }
                index++;
            }
            return count;
        }

        public int minimizeMax(int[] nums, int p) {
            Arrays.sort(nums);
            int n = nums.length;
            int left = 0, right = nums[n - 1] - nums[0];

            while (left < right) {
                int mid = left + (right - left) / 2;
                if (countValidPairs(nums, mid) >= p) {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            return left;
        }
    }

}
