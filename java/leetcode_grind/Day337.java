package leetcode_grind;

import java.util.Arrays;

public class Day337 {
    // https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps/description/
    static class Solution1 {
        public int numWays(int steps, int arrLen) {
            int MOD = 1000_000_007;
            arrLen = Math.min(arrLen, steps);
            var dp = new int[arrLen];
            var prevDp = new int[arrLen];
            prevDp[0] = 1;

            for (int remain = 1; remain <= steps; remain++) {
                dp = new int[arrLen];
                for (int curr = arrLen - 1; curr >= 0; curr--) {
                    var ans = prevDp[curr];
                    if (curr > 0) {
                        ans = (ans + prevDp[curr - 1]) % MOD;
                    }
                    if (curr < arrLen - 1) {
                        ans = (ans + prevDp[curr + 1]) % MOD;
                    }
                    dp[curr] = ans;
                }
                prevDp = dp;
            }

            return dp[0];
        }
    }

    static class Solution2 {
        public int paintWalls(int[] cost, int[] time) {
            var n = cost.length;
            var dp = new int[n + 1];
            var prevDp = new int[n + 1];
            Arrays.fill(prevDp, 1000000000);
            prevDp[0] = 0;

            for (int i = n - 1; i >= 0; i--) {
                dp = new int[n + 1];
                for (int remain = 1; remain <= n; remain++) {
                    var paint = cost[i] + prevDp[Math.max(0, remain - 1 - time[i])];
                    var dontPaint = prevDp[remain];
                    dp[remain] = Math.min(paint, dontPaint);
                }
                prevDp = dp;
            }
            return dp[n];
        }
    }
}
