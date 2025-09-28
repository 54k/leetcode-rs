package leetcode_grind;

import java.util.Arrays;

public class Day1044 {
    // https://leetcode.com/problems/largest-perimeter-triangle/description/?envType=daily-question&envId=2025-09-28
    static class Solution1 {
        public int largestPerimeter(int[] A) {
            Arrays.sort(A);
            for (int i = A.length - 3; i >= 0; i--) {
                if (A[i] + A[i + 1] > A[i + 2]) {
                    return A[i] + A[i + 1] + A[i + 2];
                }
            }
            return 0;
        }
    }

    // https://leetcode.com/problems/coin-change/description/
    static class Solution2 {
        public int coinChange(int[] coins, int amount) {
            int[] dp = new int[amount + 1];
            Arrays.fill(dp, amount + 1);

            dp[0] = 0;
            for (int i = 1; i <= amount; i++) {
                for (int coin : coins) {
                    if (i - coin < 0)
                        continue;
                    dp[i] = Math.min(dp[i], dp[i - coin] + 1);
                }
            }

            return dp[amount] == (amount + 1) ? -1 : dp[amount];
        }
    }
}
