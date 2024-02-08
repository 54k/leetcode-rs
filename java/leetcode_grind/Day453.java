package leetcode_grind;

public class Day453 {
    // https://leetcode.com/problems/paint-fence/description
    static class Solution1 {
        public int numWays(int n, int k) {
            int[] dp = new int[n + 1];
            dp[0] = k;
            dp[1] = k * k;

            for (int i = 2; i < n; i++) {
                dp[i] = (k - 1) * (dp[i - 1] + dp[i - 2]);
            }

            return dp[n - 1];
        }
    }
}
