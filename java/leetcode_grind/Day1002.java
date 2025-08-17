package leetcode_grind;

public class Day1002 {
    // https://leetcode.com/problems/new-21-game/description/?envType=daily-question&envId=2025-08-17
    static class Solution1 {
        public double new21Game(int n, int k, int maxPts) {
            double dp[] = new double[n + 1];
            dp[0] = 1;
            double s = k > 0 ? 1 : 0;
            for (int i = 1; i <= n; i++) {
                dp[i] = s / maxPts;
                if (i < k) {
                    s += dp[i];
                }
                if (i - maxPts >= 0 && i - maxPts < k) {
                    s -= dp[i - maxPts];
                }
            }
            double ans = 0;
            for (int i = k; i <= n; i++) {
                ans += dp[i];
            }
            return ans;
        }
    }
}
