package data_structures_examples;

public class BinSearch {
    // https://leetcode.com/problems/search-insert-position/description/
    static class Solution1 {
        public int searchInsert1(int[] nums, int target) {
            var lo = -1;
            var hi = nums.length;

            while (hi - lo > 1) {
                var mid = (lo + hi) / 2;

                if (nums[mid] < target) {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            return hi;
        }

        public int searchInsert2(int[] nums, int target) {
            var lo = 0;
            var hi = nums.length;
            while (lo < hi) {
                var mid = (lo + hi) / 2;
                if (nums[mid] < target) {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            return lo;
        }

        public int searchInsert3(int[] nums, int target) {
            var lo = 0;
            var hi = nums.length - 1;
            while (lo <= hi) {
                var mid = (lo + hi) / 2;
                if (nums[mid] == target) {
                    return mid;
                }
                if (nums[mid] < target) {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
            return lo;
        }
    }
}
