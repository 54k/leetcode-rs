package leetcode_grind;

public class Day330 {
    // https://leetcode.com/problems/max-dot-product-of-two-subsequences/description/
    static class Solution {
        public int maxDotProduct(int[] nums1, int[] nums2) {
            var maxNum1 = Integer.MIN_VALUE;
            var minNum1 = Integer.MAX_VALUE;
            for (var n : nums1) {
                maxNum1 = Math.max(maxNum1, n);
                minNum1 = Math.min(minNum1, n);
            }

            var maxNum2 = Integer.MIN_VALUE;
            var minNum2 = Integer.MAX_VALUE;
            for (var n : nums2) {
                maxNum2 = Math.max(maxNum2, n);
                minNum2 = Math.min(minNum2, n);
            }

            if (maxNum1 < 0 && minNum2 > 0) {
                return maxNum1 * minNum2;
            }
            if (minNum1 > 0 && maxNum2 < 0) {
                return minNum1 * maxNum2;
            }

            var m = nums2.length + 1;
            var prevDp = new int[m];
            var dp = new int[m];

            for (int i = nums1.length - 1; i >= 0; i--) {
                dp = new int[m];
                for (int j = nums2.length - 1; j >= 0; j--) {
                    var use = nums1[i] * nums2[j] + prevDp[j + 1];
                    dp[j] = Math.max(use, Math.max(prevDp[j], dp[j + 1]));
                }
                prevDp = dp;
            }

            return dp[0];
        }
    }
}
