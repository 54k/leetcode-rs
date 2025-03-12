package leetcode_grind;

public class Day844 {

    // https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer/description/?envType=daily-question&envId=2025-03-12
    static class Solution1 {
        int lowerBound(int[] nums) {
            int start = 0, end = nums.length - 1;
            int index = nums.length;

            while (start <= end) {
                int mid = (start + end) / 2;
                if (nums[mid] < 0) {
                    start = mid + 1;
                } else if (nums[mid] >= 0) {
                    end = mid - 1;
                    index = mid;
                }
            }
            return index;
        }

        int upperBound(int[] nums) {
            int start = 0, end = nums.length - 1;
            int index = nums.length;

            while (start <= end) {
                int mid = (start + end) / 2;
                if (nums[mid] <= 0) {
                    start = mid + 1;
                } else if (nums[mid] > 0) {
                    end = mid - 1;
                    index = mid;
                }
            }
            return index;
        }

        public int maximumCount(int[] nums) {
            int positiveCount = nums.length - upperBound(nums);
            int negativeCount = lowerBound(nums);
            return Math.max(positiveCount, negativeCount);
        }
    }

    static class Solution2 {
        int lowerBound(int[] nums) {
            int lo = -1, hi = nums.length;
            while (lo + 1 < hi) {
                int mid = (lo + hi) / 2;
                if (nums[mid] < 0) {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            return lo;
        }

        int upperBound(int[] nums) {
            int lo = -1, hi = nums.length;
            while (lo + 1 < hi) {
                int mid = (lo + hi) / 2;
                if (nums[mid] <= 0) {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            return hi;
        }

        public int maximumCount(int[] nums) {
            int positiveCount = nums.length - upperBound(nums);
            int negativeCount = lowerBound(nums) + 1;
            return Math.max(positiveCount, negativeCount);
        }
    }
}
