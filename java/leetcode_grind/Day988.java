package leetcode_grind;

public class Day988 {
    // https://leetcode.com/problems/maximum-fruits-harvested-after-at-most-k-steps/description/?envType=daily-question&envId=2025-08-03
    static class Solution1 {
        public int maxTotalFruits(int[][] fruits, int startPos, int k) {
            int n = fruits.length;

            int[] sum = new int[n + 1];
            int[] indices = new int[n];

            sum[0] = 0;
            for (int i = 0; i < n; i++) {
                sum[i + 1] = sum[i] + fruits[i][1];
                indices[i] = fruits[i][0];
            }

            int ans = 0;
            for (int x = 0; x <= k / 2; x++) {
                int y = k - 2 * x;
                int left = startPos - x;
                int right = startPos + y;
                int start = lowerBound(indices, 0, n - 1, left);
                int end = upperBound(indices, 0, n - 1, right);
                ans = Math.max(ans, sum[end] - sum[start]);

                y = k - 2 * x;
                left = startPos - y;
                right = startPos + x;
                start = lowerBound(indices, 0, n - 1, left);
                end = upperBound(indices, 0, n - 1, right);
                ans = Math.max(ans, sum[end] - sum[start]);
            }
            return ans;
        }

        int lowerBound(int[] arr, int left, int right, int val) {
            int res = right + 1;
            while (left <= right) {
                int mid = left + (right - left) / 2;
                if (arr[mid] >= val) {
                    res = mid;
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            return res;
        }

        int upperBound(int[] arr, int left, int right, int val) {
            int res = right + 1;
            while (left <= right) {
                int mid = left + (right - left) / 2;
                if (arr[mid] > val) {
                    res = mid;
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            return res;
        }
    }

    static class Solution2 {
        public int maxTotalFruits(int[][] fruits, int startPos, int k) {
            int n = fruits.length;

            int[] sum = new int[n + 1];
            int[] indices = new int[n];

            sum[0] = 0;
            for (int i = 0; i < n; i++) {
                sum[i + 1] = sum[i] + fruits[i][1];
                indices[i] = fruits[i][0];
            }

            int ans = 0;
            for (int x = 0; x <= k / 2; x++) {
                int y = k - 2 * x;
                int left = startPos - x;
                int right = startPos + y;
                int start = lowerBound(indices, -1, n, left);
                int end = upperBound(indices, -1, n, right);
                ans = Math.max(ans, sum[end] - sum[start]);

                y = k - 2 * x;
                left = startPos - y;
                right = startPos + x;
                start = lowerBound(indices, -1, n, left);
                end = upperBound(indices, -1, n, right);
                ans = Math.max(ans, sum[end] - sum[start]);
            }
            return ans;
        }

        int lowerBound(int[] arr, int left, int right, int val) {
            while (left + 1 < right) {
                int mid = left + (right - left) / 2;
                if (arr[mid] < val) {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            return right;
        }

        int upperBound(int[] arr, int left, int right, int val) {
            while (left + 1 < right) {
                int mid = left + (right - left) / 2;
                if (arr[mid] <= val) {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            return right;
        }
    }

    static class Solution3 {
        public int maxTotalFruits(int[][] fruits, int startPos, int k) {
            int left = 0;
            int right = 0;

            int n = fruits.length;
            int sum = 0;
            int ans = 0;

            while (right < n) {
                sum += fruits[right][1];
                while (left <= right && step(fruits, startPos, left, right) > k) {
                    sum -= fruits[left][1];
                    left++;
                }
                ans = Math.max(ans, sum);
                right++;
            }
            return ans;
        }

        int step(int[][] fruits, int startPos, int left, int right) {
            return (Math.min(Math.abs(startPos - fruits[right][0]), Math.abs(startPos - fruits[left][0]))
                    + fruits[right][0] - fruits[left][0]);
        }
    }
}
