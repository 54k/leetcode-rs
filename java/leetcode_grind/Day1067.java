package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

public class Day1067 {
    // https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-i/description/?envType=daily-question&envId=2025-10-21
    static class Solution1 {
        public int maxFrequency(int[] nums, int k, int numOperations) {
            int n = nums.length, ans = 0, left = 0, right = 0;
            Arrays.sort(nums);

            Map<Integer, Integer> count = new HashMap<>();
            for (int num : nums)
                count.put(num, count.getOrDefault(num, 0) + 1);

            for (int mid = 0; mid < n; mid++) {
                while (nums[mid] - nums[left] > k) {
                    left++;
                }

                while (right < n - 1 && nums[right + 1] - nums[mid] <= k) {
                    right++;
                }

                int total = right - left + 1;
                ans = Math.max(ans, Math.min(total - count.get(nums[mid]), numOperations) + count.get(nums[mid]));
            }

            left = 0;
            for (right = 0; right < n; right++) {
                int mid = (nums[left] + nums[right]) / 2;
                while (mid - nums[left] > k || nums[right] - mid > k) {
                    left++;
                    mid = (nums[left] + nums[right]) / 2;
                }
                ans = Math.max(ans, Math.min(right - left + 1, numOperations));
            }
            return ans;
        }
    }

    static class Solution2 {

        public int maxFrequency(int[] nums, int k, int numOperations) {
            Arrays.sort(nums);

            int ans = 0;
            Map<Integer, Integer> numCount = new HashMap<>();

            int lastNumIndex = 0;
            for (int i = 0; i < nums.length; ++i) {
                if (nums[i] != nums[lastNumIndex]) {
                    numCount.put(nums[lastNumIndex], i - lastNumIndex);
                    ans = Math.max(ans, i - lastNumIndex);
                    lastNumIndex = i;
                }
            }

            numCount.put(nums[lastNumIndex], nums.length - lastNumIndex);
            ans = Math.max(ans, nums.length - lastNumIndex);

            for (int i = nums[0]; i <= nums[nums.length - 1]; i++) {
                int l = leftBound(nums, i - k);
                int r = rightBound(nums, i + k);
                int tempAns;
                if (numCount.containsKey(i)) {
                    tempAns = Math.min(r - l + 1, numCount.get(i) + numOperations);
                } else {
                    tempAns = Math.min(r - l + 1, numOperations);
                }
                ans = Math.max(ans, tempAns);
            }

            return ans;
        }

        private int leftBound(int[] nums, int value) {
            int left = 0;
            int right = nums.length - 1;
            while (left < right) {
                int mid = (left + right) / 2;
                if (nums[mid] < value) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            return left;
        }

        private int rightBound(int[] nums, int value) {
            int left = 0;
            int right = nums.length - 1;
            while (left < right) {
                int mid = (left + right + 1) / 2;
                if (nums[mid] > value) {
                    right = mid - 1;
                } else {
                    left = mid;
                }
            }
            return left;
        }
    }

    // https://leetcode.com/problems/range-sum-query-2d-mutable/description/
    static class NumMatrix {
        int rows;
        int cols;
        int[][] bit;

        int lsb(int n) {
            return n & (-n);
        }

        void updateBIT(int r, int c, int val) {
            for (int i = r; i <= rows; i += lsb(i)) {
                for (int j = c; j <= cols; j += lsb(j)) {
                    this.bit[i][j] += val;
                }
            }
        }

        int queryBIT(int r, int c) {
            int sum = 0;
            for (int i = r; i > 0; i -= lsb(i)) {
                for (int j = c; j > 0; j -= lsb(j)) {
                    sum += this.bit[i][j];
                }
            }
            return sum;
        }

        void buildBIT(int[][] matrix) {
            for (int i = 1; i <= rows; ++i) {
                for (int j = 1; j <= cols; ++j) {
                    int val = matrix[i - 1][j - 1];
                    updateBIT(i, j, val);
                }
            }
        }

        public NumMatrix(int[][] matrix) {
            rows = matrix.length;
            if (rows == 0)
                return;
            cols = matrix[0].length;
            bit = new int[rows + 1][];
            for (int i = 1; i <= rows; i++) {
                bit[i] = new int[cols + 1];
            }
            buildBIT(matrix);
        }

        public void update(int row, int col, int val) {
            int old_val = sumRegion(row, col, row, col);
            row++;
            col++;
            int diff = val - old_val;
            updateBIT(row, col, diff);
        }

        public int sumRegion(int row1, int col1, int row2, int col2) {
            row1++;
            col1++;
            row2++;
            col2++;
            int a = queryBIT(row2, col2);
            int b = queryBIT(row1 - 1, col1 - 1);
            int c = queryBIT(row2, col1 - 1);
            int d = queryBIT(row1 - 1, col2);
            return (a + b) - (c + d);
        }
    }

    /**
     * Your NumMatrix object will be instantiated and called as such:
     * NumMatrix obj = new NumMatrix(matrix);
     * obj.update(row,col,val);
     * int param_2 = obj.sumRegion(row1,col1,row2,col2);
     */
}
