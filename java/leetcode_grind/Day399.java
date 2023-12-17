package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day399 {
    // https://leetcode.com/problems/valid-anagram/description/
    static class Solution {
        public boolean isAnagram(String s, String t) {
            Map<Character, Integer> freq = new HashMap<>();
            for (var ch : s.toCharArray()) {
                freq.put(ch, freq.getOrDefault(ch, 0) + 1);
            }

            for (var ch : t.toCharArray()) {
                freq.put(ch, freq.getOrDefault(ch, 0) - 1);
                if (freq.get(ch) == 0) {
                    freq.remove(ch);
                }
            }

            return freq.isEmpty();
        }
    }

}
