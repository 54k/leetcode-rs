package leetcode_grind;

import java.util.*;

public class Day777 {
    // https://leetcode.com/problems/unique-length-3-palindromic-subsequences/description/?envType=daily-question&envId=2025-01-04
    static class Solution1 {
        public int countPalindromicSubsequence(String s) {
            Set<Character> letters = new HashSet<>();
            for (Character c : s.toCharArray()) {
                letters.add(c);
            }

            int ans = 0;
            for (Character letter : letters) {
                int i = -1;
                int j = 0;

                for (int k = 0; k < s.length(); k++) {
                    if (s.charAt(k) == letter) {
                        if (i == -1) {
                            i = k;
                        }

                        j = k;
                    }
                }

                Set<Character> between = new HashSet<>();
                for (int k = i + 1; k < j; k++) {
                    between.add(s.charAt(k));
                }
                ans += between.size();
            }
            return ans;
        }
    }

    static class Solution2 {
        public int countPalindromicSubsequence(String s) {
            int[] first = new int[26];
            int[] last = new int[26];
            Arrays.fill(first, -1);

            for (int i = 0; i < s.length(); i++) {
                int curr = s.charAt(i) - 'a';
                if (first[curr] == -1) {
                    first[curr] = i;
                }
                last[curr] = i;
            }

            int ans = 0;
            for (int i = 0; i < 26; i++) {
                if (first[i] == -1) {
                    continue;
                }

                Set<Character> between = new HashSet<>();
                for (int j = first[i] + 1; j < last[i]; j++) {
                    between.add(s.charAt(j));
                }

                ans += between.size();
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/strings-differ-by-one-character/description/
    static class Solution3 {
        public boolean differByOne(String[] dict) {
            HashSet<Long> set = new HashSet<>();
            long mod = (long) Math.pow(10, 20) + 7;
            int len = dict[0].length();
            long[] word2hash = new long[dict.length];
            for (int i = 0; i < dict.length; i++) {
                for (int j = 0; j < len; j++) {
                    word2hash[i] = (word2hash[i] * 26 + dict[i].charAt(j) - 'a') % mod;
                }
            }

            long base = 1;
            for (int j = len - 1; j >= 0; j--) {
                set.clear();

                for (int i = 0; i < dict.length; i++) {
                    long newHash = (word2hash[i] - base * (dict[i].charAt(j) - 'a')) % mod;
                    if (set.contains(newHash)) {
                        return true;
                    }
                    set.add(newHash);
                }
                base = 26 * base % mod;
            }

            return false;
        }
    }
}
