package leetcode_grind;

public class Day401 {
    // https://leetcode.com/problems/number-of-longest-increasing-subsequence/description
    static class Solution {
        public int findNumberOfLIS(int[] nums) {
            int[][] dp = new int[nums.length][2];
            int longest = 1;

            for (int i = 0; i < nums.length; i++) {
                dp[i] = new int[] { 1, 1 };
                for (int j = 0; j < i; j++) {
                    if (nums[j] < nums[i]) {
                        if (dp[i][0] < dp[j][0] + 1) {
                            dp[i] = new int[] { dp[j][0] + 1, dp[j][1] };
                            longest = Math.max(longest, dp[i][0]);
                        } else if (dp[i][0] - 1 == dp[j][0]) {
                            dp[i][1] += dp[j][1];
                        }
                    }
                }
            }

            int ans = 0;
            for (int i = 0; i < nums.length; i++) {
                if (longest == dp[i][0]) {
                    ans += dp[i][1];
                }
            }

            return ans;
        }
    }

}
