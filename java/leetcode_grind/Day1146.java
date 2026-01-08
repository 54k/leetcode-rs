package leetcode_grind;

public class Day1146 {
    // https://leetcode.com/problems/max-dot-product-of-two-subsequences/description/?envType=daily-question&envId=2026-01-08
    static class Solution1 {
        int[][] memo;

        int dp(int i, int j, int[] nums1, int[] nums2) {
            if (i == nums1.length || j == nums2.length) {
                return 0;
            }

            if (memo[i][j] != 0) {
                return memo[i][j];
            }

            int use = nums1[i] * nums2[j] + dp(i + 1, j + 1, nums1, nums2);
            memo[i][j] = Math.max(use, Math.max(dp(i + 1, j, nums1, nums2), dp(i, j + 1, nums1, nums2)));
            return memo[i][j];
        }

        public int maxDotProduct(int[] nums1, int[] nums2) {
            int firstMax = Integer.MIN_VALUE;
            int secondMax = Integer.MIN_VALUE;
            int firstMin = Integer.MAX_VALUE;
            int secondMin = Integer.MAX_VALUE;

            for (int num : nums1) {
                firstMax = Math.max(firstMax, num);
                firstMin = Math.min(firstMin, num);
            }

            for (int num : nums2) {
                secondMax = Math.max(secondMax, num);
                secondMin = Math.min(secondMin, num);
            }

            if (firstMax < 0 && secondMin > 0) {
                return firstMax * secondMin;
            }

            if (firstMin > 0 && secondMax < 0) {
                return firstMin * secondMax;
            }

            memo = new int[nums1.length][nums2.length];
            return dp(0, 0, nums1, nums2);
        }
    }

    static class Solution2 {
        public int maxDotProduct(int[] nums1, int[] nums2) {
            int firstMax = Integer.MIN_VALUE;
            int secondMax = Integer.MIN_VALUE;
            int firstMin = Integer.MAX_VALUE;
            int secondMin = Integer.MAX_VALUE;

            for (int num : nums1) {
                firstMax = Math.max(firstMax, num);
                firstMin = Math.min(firstMin, num);
            }

            for (int num : nums2) {
                secondMax = Math.max(secondMax, num);
                secondMin = Math.min(secondMin, num);
            }

            if (firstMax < 0 && secondMin > 0) {
                return firstMax * secondMin;
            }

            if (firstMin > 0 && secondMax < 0) {
                return firstMin * secondMax;
            }

            int[][] dp = new int[nums1.length + 1][nums2.length + 1];
            for (int i = nums1.length - 1; i >= 0; i--) {
                for (int j = nums2.length - 1; j >= 0; j--) {
                    int use = nums1[i] * nums2[j] + dp[i + 1][j + 1];
                    dp[i][j] = Math.max(use, Math.max(dp[i + 1][j], dp[i][j + 1]));
                }
            }
            return dp[0][0];
        }
    }

}
