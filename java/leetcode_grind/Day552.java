package leetcode_grind;

public class Day552 {
    // https://leetcode.com/problems/find-the-maximum-sum-of-node-values/description
    static class Solution1 {
        public long maximumValueSum(int[] nums, int k, int[][] edges) {
            var n = nums.length;

            var dp = new long[n + 1][2];
            dp[n][1] = 0;
            dp[n][0] = -(1 << 31);

            for (var i = n - 1; i >= 0; i--) {
                for (var j = 0; j < 2; j++) {
                    var xor = (nums[i] ^ k) + dp[i + 1][j ^ 1];
                    dp[i][j] = Math.max(xor, nums[i] + dp[i + 1][j]);
                }
            }

            return dp[0][1];
        }
    }
}