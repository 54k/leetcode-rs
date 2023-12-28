package leetcode_grind;

import java.util.HashMap;
import java.util.Map;
import java.util.Set;

public class Day411 {
    // https://leetcode.com/problems/string-compression-ii/description
    static class Solution1 {
        Map<Integer, Integer> memo = new HashMap<>();
        Set<Integer> add = Set.of(1, 9, 99);

        public int getLengthOfOptimalCompression(String s, int k) {
            return dp(s, 0, (char) ('a' + 26), 0, k);
        }

        int dp(String s, int idx, char lastChar, int lastCharCount, int k) {
            if (k < 0) {
                return Integer.MAX_VALUE / 2;
            }

            if (idx == s.length()) {
                return 0;
            }

            int key = idx * 101 * 27 * 101 + (lastChar - 'a') * 101 * 101 + lastCharCount * 101 + k;

            if (memo.containsKey(key)) {
                return memo.get(key);
            }

            int keepChar;
            int deleteChar = dp(s, idx + 1, lastChar, lastCharCount, k - 1);
            if (s.charAt(idx) == lastChar) {
                keepChar = dp(s, idx + 1, lastChar, lastCharCount + 1, k) + (add.contains(lastCharCount) ? 1 : 0);
            } else {
                keepChar = dp(s, idx + 1, s.charAt(idx), 1, k) + 1;
            }

            int res = Math.min(keepChar, deleteChar);
            memo.put(key, res);
            return res;
        }
    }

    // https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/description
    static class Solution2 {
        public int minInsertions(String s) {
            var lcs = new Object() {
                int apply(String s1, String s2) {
                    var m = s1.length();
                    var n = s2.length();
                    var dp = new int[m + 1][n + 1];
                    for (int i = 1; i <= m; i++) {
                        for (int j = 1; j <= n; j++) {
                            if (s1.charAt(i - 1) == s2.charAt(j - 1)) {
                                dp[i][j] = dp[i - 1][j - 1] + 1;
                            } else {
                                dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1]);
                            }
                        }
                    }
                    return dp[m][n];
                }
            };
            var arr = s.toCharArray();
            for (int i = 0; i < arr.length / 2; i++) {
                var tmp = arr[i];
                arr[i] = arr[arr.length - i - 1];
                arr[arr.length - i - 1] = tmp;
            }
            return s.length() - lcs.apply(s, new String(arr));
        }
    }
}
