package leetcode_grind;

import java.util.Arrays;

public class Day641 {
    // https://leetcode.com/problems/2-keys-keyboard/description/?envType=daily-question&envId=2024-08-19
    static class Solution1 {
        int n;

        public int minSteps(int n) {
            if (n == 1)
                return 0;
            this.n = n;
            return 1 + minStepsHelper(1, 1);
        }

        int minStepsHelper(int currLen, int pasteLen) {
            if (currLen == n)
                return 0;
            if (currLen > n)
                return 1000;
            int opt1 = 2 + minStepsHelper(currLen * 2, currLen);
            int opt2 = 1 + minStepsHelper(currLen + pasteLen, pasteLen);
            return Math.min(opt1, opt2);
        }
    }

    static class Solution2 {
        int n;

        public int minSteps(int n) {
            if (n == 1)
                return 0;
            this.n = n;
            int[][] memo = new int[n + 1][n / 2 + 1];
            return 1 + minStepsHelper(1, 1, memo);
        }

        int minStepsHelper(int currLen, int pasteLen, int[][] memo) {
            if (currLen == n)
                return 0;
            if (currLen > n)
                return 1000;
            if (memo[currLen][pasteLen] != 0)
                return memo[currLen][pasteLen];
            int opt1 = 2 + minStepsHelper(currLen * 2, currLen, memo);
            int opt2 = 1 + minStepsHelper(currLen + pasteLen, pasteLen, memo);
            return memo[currLen][pasteLen] = Math.min(opt1, opt2);
        }
    }

    static class Solution3 {
        public int minSteps(int n) {
            int[] dp = new int[n + 1];
            Arrays.fill(dp, 1000);
            dp[1] = 0;
            for (int i = 2; i <= n; i++) {
                for (int j = 1; j <= i / 2; j++) {
                    if (i % j == 0) {
                        dp[i] = Math.min(dp[i], dp[j] + i / j);
                    }
                }
            }
            return dp[n];
        }
    }

    static class Solution4 {
        public int minSteps(int n) {
            int ans = 0;
            int d = 2;
            while (n > 1) {
                while (n % d == 0) {
                    ans += d;
                    n /= d;
                }
                d++;
            }
            return ans;
        }
    }
}
