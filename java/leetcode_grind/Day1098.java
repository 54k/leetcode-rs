package leetcode_grind;

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

}
