package leetcode_grind;

import java.util.Arrays;
import java.util.HashSet;
import java.util.List;

public class Day367 {
    // https://leetcode.com/problems/unique-length-3-palindromic-subsequences/description
    static class Solution1 {
        public int countPalindromicSubsequence(String s) {
            var ans = 0;
            var letters = new HashSet<Character>();
            for (var let : s.toCharArray()) {
                letters.add(let);
            }
            for (var let : letters) {
                var j = -1;
                var i = 0;

                for (int k = 0; k < s.length(); k++) {
                    if (let == s.charAt(k)) {
                        if (j == -1) {
                            j = k;
                        }
                        i = k;
                    }
                }

                var between = new HashSet<Character>();
                for (int k = j + 1; k < i; k++) {
                    between.add(s.charAt(k));
                }
                ans += between.size();
            }
            return ans;
        }

        public int countPalindromicSubsequence2(String s) {
            var ans = 0;

            var first = new int[26];
            Arrays.fill(first, -1);
            var last = new int[26];
            Arrays.fill(last, -1);

            for (int i = 0; i < s.length(); i++) {
                var ch = s.charAt(i);
                if (first[ch - 'a'] == -1) {
                    first[ch - 'a'] = i;
                }
                last[ch - 'a'] = i;
            }

            for (int i = 0; i < 26; i++) {
                if (first[i] == -1) {
                    continue;
                }

                var between = new HashSet<Character>();
                for (int k = first[i] + 1; k < last[i]; k++) {
                    var ch = s.charAt(k);
                    between.add(ch);
                }

                ans += between.size();
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/longest-binary-subsequence-less-than-or-equal-to-k/description/
    static class Solution2 {
        public int longestSubsequence1(String s, int k) {
            var dp = new int[s.length() + 1];
            var j = 0;
            for (char c : s.toCharArray()) {
                if (dp[j] * 2 + c - '0' <= k) {
                    dp[j + 1] = dp[j] * 2 + c - '0';
                    j++;
                }
                for (int i = j; i > 0; --i) {
                    dp[i] = Math.min(dp[i], dp[i - 1] * 2 + c - '0');
                }
            }
            return j;
        }

        public int longestSubsequence2(String s, int k) {
            var res = 0;
            var cost = 1;
            var n = s.length();

            for (int i = n - 1; i >= 0; --i) {
                if (s.charAt(i) == '0' || cost <= k) {
                    k -= cost * (s.charAt(i) - '0');
                    res++;
                }
                if (cost <= k) {
                    cost *= 2;
                }
            }

            return res;
        }
    }

    // https://leetcode.com/problems/word-break/description
    class Solution3 {
        public boolean wordBreak(String s, List<String> wordDict) {
            var set = new HashSet<String>(wordDict);
            var dp = new boolean[s.length() + 1];
            dp[s.length()] = true;

            for (int i = s.length(); i >= 0; i--) {
                for (int j = i + 1; j <= s.length(); j++) {
                    if (!dp[j]) {
                        continue;
                    }

                    var substr = s.substring(i, j);
                    dp[i] = set.contains(substr);
                    if (dp[i]) {
                        break;
                    }
                }
            }

            return dp[0];
        }
    }
}
