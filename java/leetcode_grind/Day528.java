package leetcode_grind;

public class Day528 {
    // https://leetcode.com/problems/restore-the-array/description/
    static class Solution1 {
        public int numberOfArrays(String s, int k) {
            int MOD = 1000000007;
            var dp = new int[s.length() + 1];
            dp[0] = 1;

            for (int i = 0; i < s.length(); i++) {
                var ch = s.charAt(i);
                if (ch == '0')
                    continue;

                for (int j = i; j < s.length(); j++) {
                    var ss = s.substring(i, j + 1);
                    if (Long.parseLong(ss) > k) {
                        break;
                    }

                    dp[j + 1] = (dp[j + 1] + dp[i]) % MOD;
                }
            }

            return dp[s.length()];
        }
    }
}
