package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day1029 {

    // https://leetcode.com/problems/find-most-frequent-vowel-and-consonant/description/?envType=daily-question&envId=2025-09-13
    static class Solution1 {

        public int maxFreqSum(String s) {
            Map<Character, Integer> mp = new HashMap<>();
            for (char ch : s.toCharArray()) {
                mp.put(ch, mp.getOrDefault(ch, 0) + 1);
            }
            int vowel = 0;
            int consonant = 0;
            for (char ch = 'a'; ch <= 'z'; ch++) {
                if (isVowel(ch)) {
                    vowel = Math.max(vowel, mp.getOrDefault(ch, 0));
                } else {
                    consonant = Math.max(consonant, mp.getOrDefault(ch, 0));
                }
            }
            return vowel + consonant;
        }

        private boolean isVowel(char c) {
            return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
        }
    }
}
