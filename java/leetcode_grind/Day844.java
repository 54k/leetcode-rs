package leetcode_grind;

import java.util.Arrays;

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

    // https://leetcode.com/problems/candy/description/
    static class Solution3 {
        public int candy(int[] ratings) {
            int[] candies = new int[ratings.length];
            Arrays.fill(candies, 1);
            boolean hasChanged = true;
            while (hasChanged) {
                hasChanged = false;
                for (int i = 0; i < ratings.length; i++) {
                    if (i != ratings.length - 1 && ratings[i] > ratings[i + 1] && candies[i] <= candies[i + 1]) {
                        candies[i] = candies[i + 1] + 1;
                        hasChanged = true;
                    }

                    if (i > 0 && ratings[i] > ratings[i - 1] && candies[i] <= candies[i - 1]) {
                        candies[i] = candies[i - 1] + 1;
                        hasChanged = true;
                    }
                }
            }
            int sum = 0;
            for (int candy : candies) {
                sum += candy;
            }
            return sum;
        }
    }

    static class Solution4 {
        public int candy(int[] ratings) {
            int sum = 0;

            int[] left2right = new int[ratings.length];
            int[] right2left = new int[ratings.length];
            Arrays.fill(left2right, 1);
            Arrays.fill(right2left, 1);

            for (int i = 1; i < ratings.length; i++) {
                if (ratings[i] > ratings[i - 1]) {
                    left2right[i] = left2right[i - 1] + 1;
                }
            }
            for (int i = ratings.length - 2; i >= 0; i--) {
                if (ratings[i] > ratings[i + 1]) {
                    right2left[i] = right2left[i + 1] + 1;
                }
            }
            for (int i = 0; i < ratings.length; i++) {
                sum += Math.max(left2right[i], right2left[i]);
            }
            return sum;
        }
    }

    static class Solution5 {
        public int candy(int[] ratings) {
            int[] candies = new int[ratings.length];
            Arrays.fill(candies, 1);
            for (int i = 1; i < ratings.length; i++) {
                if (ratings[i] > ratings[i - 1]) {
                    candies[i] = candies[i - 1] + 1;
                }
            }
            int sum = candies[ratings.length - 1];
            for (int i = ratings.length - 2; i >= 0; i--) {
                if (ratings[i] > ratings[i + 1]) {
                    candies[i] = Math.max(candies[i], candies[i + 1] + 1);
                }
                sum += candies[i];
            }
            return sum;
        }
    }
}
