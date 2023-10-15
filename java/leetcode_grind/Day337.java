package leetcode_grind;

public class Day337 {
    // https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps/description/
    class Solution {
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
}
