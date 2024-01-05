package leetcode_grind;

import java.util.Arrays;

public class Day419 {
    // https://leetcode.com/problems/maximum-length-of-pair-chain/description/
    static class Solution1 {
        public int findLongestChain(int[][] pairs) {
            int n = pairs.length;
            Arrays.sort(pairs, (a, b) -> a[1] - b[1]);
            int ans = 0;
            int cur = Integer.MIN_VALUE;
            for (int i = 0; i < n; i++) {
                if (pairs[i][0] > cur) {
                    cur = pairs[i][1];
                    ans++;
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/longest-ideal-subsequence/
    static class Solution2 {
        public int longestIdealString(String s, int k) {
            int n = s.length();
            var dp = new int[150];
            int mx = 1;

            for (int i = 0; i < n; i++) {
                int ch = s.charAt(i);

                for (int j = ch - k; j <= ch + k; j++) {
                    dp[ch] = Math.max(dp[ch], dp[j]);
                }

                mx = Math.max(++dp[ch], mx);
            }

            return mx;
        }
    }
}
