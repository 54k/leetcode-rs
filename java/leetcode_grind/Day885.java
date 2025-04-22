package leetcode_grind;

import java.util.Arrays;

public class Day885 {
    // https://leetcode.com/problems/smallest-rectangle-enclosing-black-pixels/description/
    static class Solution1 {
        public int minArea(char[][] image, int x, int y) {
            int m = image.length, n = image[0].length;
            int left = searchColumns(image, 0, y, 0, m, true);
            int right = searchColumns(image, y + 1, n, 0, m, false);
            int top = searchRows(image, 0, x, left, right, true);
            int bottom = searchRows(image, x + 1, m, left, right, false);
            return (right - left) * (bottom - top);
        }

        int searchColumns(char[][] image, int i, int j, int top, int bottom, boolean whiteToBlack) {
            while (i != j) {
                int k = top, mid = (i + j) / 2;
                while (k < bottom && image[k][mid] == '0')
                    ++k;
                if (k < bottom == whiteToBlack) {
                    j = mid;
                } else {
                    i = mid + 1;
                }
            }
            return i;
        }

        int searchRows(char[][] image, int i, int j, int left, int right, boolean whiteToBlack) {
            while (i != j) {
                int k = left, mid = (i + j) / 2;
                while (k < right && image[mid][k] == '0')
                    ++k;
                if (k < right == whiteToBlack) {
                    j = mid;
                } else {
                    i = mid + 1;
                }
            }
            return i;
        }
    }

    static class Solution2 {
        public int minArea(char[][] image, int x, int y) {
            int top = x, bottom = x;
            int left = y, right = y;
            for (x = 0; x < image.length; x++) {
                for (y = 0; y < image[0].length; y++) {
                    if (image[x][y] == '1') {
                        top = Math.min(top, x);
                        bottom = Math.max(bottom, x + 1);
                        left = Math.min(left, y);
                        right = Math.max(right, y + 1);
                    }
                }
            }
            return (right - left) * (bottom - top);
        }
    }

    static class Solution3 {
        public int minArea(char[][] image, int x, int y) {
            int top = x, bottom = x;
            int left = y, right = y;
            for (x = 0; x < image.length; x++) {
                for (y = 0; y < image[0].length; y++) {
                    if (image[x][y] == '1') {
                        top = Math.min(top, x);
                        bottom = Math.max(bottom, x + 1);
                        left = Math.min(left, y);
                        right = Math.max(right, y + 1);
                    }
                }
            }
            return (right - left) * (bottom - top);
        }
    }

    // https://leetcode.com/problems/count-the-number-of-fair-pairs/description/
    static class Solution4 {
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

    static class Solution5 {
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
}
