package leetcode_grind;

import java.util.Arrays;
import java.util.HashSet;

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
}
