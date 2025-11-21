package leetcode_grind;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;

public class Day1098 {
    // https://leetcode.com/problems/unique-length-3-palindromic-subsequences/description/?envType=daily-question&envId=2025-11-21
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
}
