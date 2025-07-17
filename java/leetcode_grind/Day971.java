package leetcode_grind;

public class Day971 {
    // https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-ii/description/?envType=daily-question&envId=2025-07-17
    static class Solution1 {
        public int maximumLength(int[] nums, int k) {
            int[][] dp = new int[k][k];
            int res = 0;
            for (int num : nums) {
                num %= k;
                for (int prev = 0; prev < k; prev++) {
                    dp[prev][num] = dp[num][prev] + 1;
                    res = Math.max(res, dp[prev][num]);
                }
            }
            return res;
        }
    }
}
