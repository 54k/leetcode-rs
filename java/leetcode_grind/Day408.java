package leetcode_grind;

import java.util.Arrays;

public class Day408 {
    // https://leetcode.com/problems/decode-ways/description/
    static class Solution1 {
        public int numDecodings1(String s) {
            var memo = new int[s.length()];
            Arrays.fill(memo, -1);

            var rec = new Object() {
                int apply(int i) {
                    if (i == s.length()) {
                        return 1;
                    }
                    if (i > s.length()) {
                        return 0;
                    }

                    int ans = 0;
                    if (s.charAt(i) == '0') {
                        return 0;
                    }

                    if (memo[i] != -1) {
                        return memo[i];
                    }

                    if (i < s.length() - 1) {
                        var num = Integer.valueOf(s.substring(i, i + 2));
                        if (num <= 26) {
                            ans += apply(i + 2);
                        }
                    }

                    ans += apply(i + 1);
                    memo[i] = ans;
                    return ans;
                }
            };

            return rec.apply(0);
        }

        public int numDecodings2(String s) {
            if (s.isEmpty()) {
                return 0;
            }

            var dp = new int[s.length() + 1];
            dp[0] = 1;
            dp[1] = s.charAt(0) == '0' ? 0 : 1;

            for (int i = 2; i < dp.length; i++) {
                if (s.charAt(i - 1) != '0') {
                    dp[i] = dp[i - 1];
                }

                int twoDigit = Integer.valueOf(s.substring(i - 2, i));
                if (twoDigit >= 10 && twoDigit <= 26) {
                    dp[i] += dp[i - 2];
                }
            }

            return dp[s.length()];
        }

        public int numDecodings3(String s) {
            if (s.isEmpty()) {
                return 0;
            }

            int n = s.length();
            int twoBack = 1;
            int oneBack = s.charAt(0) == '0' ? 0 : 1;

            for (int i = 1; i < n; i++) {
                int current = 0;
                if (s.charAt(i) != '0') {
                    current = oneBack;
                }
                int twoDigit = Integer.parseInt(s.substring(i - 1, i + 1));
                if (twoDigit >= 10 && twoDigit <= 26) {
                    current += twoBack;
                }

                twoBack = oneBack;
                oneBack = current;
            }

            return oneBack;
        }
    }
}
