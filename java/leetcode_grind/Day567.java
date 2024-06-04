package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day567 {
    // https://leetcode.com/problems/longest-palindrome/description
    static class Solution1 {
        public int longestPalindrome(String s) {
            Map<Character, Integer> frequencyMap = new HashMap<>();
            for (char c : s.toCharArray()) {
                frequencyMap.put(c, frequencyMap.getOrDefault(c, 0) + 1);
            }

            int res = 0;
            boolean hasOddFrequency = false;
            for (int freq : frequencyMap.values()) {
                if ((freq % 2) == 0) {
                    res += freq;
                } else {
                    res += freq - 1;
                    hasOddFrequency = true;
                }
            }
            if (hasOddFrequency)
                return res + 1;
            return res;
        }
    }

    // https://leetcode.com/problems/longest-palindrome/description
    static class Solution2 {
        public int longestPalindrome(String s) {
            int oddFreqCharsCount = 0;
            Map<Character, Integer> frequencyMap = new HashMap<>();

            for (char c : s.toCharArray()) {
                frequencyMap.put(c, frequencyMap.getOrDefault(c, 0) + 1);
                if ((frequencyMap.get(c) % 2) == 1) {
                    oddFreqCharsCount++;
                } else {
                    oddFreqCharsCount--;
                }
            }

            if (oddFreqCharsCount > 0) {
                return s.length() - oddFreqCharsCount + 1;
            }
            return s.length();
        }
    }


    
}
