package leetcode_grind;

import java.util.Arrays;

public class Day1075 {
    // https://leetcode.com/problems/minimum-subarrays-in-a-valid-split/description/?envType=weekly-question&envId=2025-10-29
    static class Solution1 {
        int INF = 1000000000;

        public int validSubarraySplit(int[] nums) {
            int n = nums.length;
            int[] dp = new int[n + 1];
            Arrays.fill(dp, INF);
            dp[0] = 0;

            for (int i = 1; i <= n; i++) {
                for (int j = 1; j <= i; j++) {
                    if (gcd(nums[i - 1], nums[j - 1]) > 1) {
                        dp[i] = Math.min(dp[i], dp[j - 1] + 1);
                    }
                }
            }

            return dp[n] == INF ? -1 : dp[n];
        }

        int gcd(int a, int b) {
            return b == 0 ? a : gcd(b, a % b);
        }
    }

    // https://leetcode.com/problems/smallest-number-with-all-set-bits/description/?envType=daily-question&envId=2025-10-29
    static class Solution2 {
        public int smallestNumber(int n) {
            int x = 1;
            while (x < n) {
                x = x * 2 + 1;
            }
            return x;
        }
    }
}
