package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day786 {
    // https://leetcode.com/problems/minimum-length-of-string-after-operations/description/?envType=daily-question&envId=2025-01-13
    static class Solution1 {
        public int minimumLength(String s) {
            Map<Character, Integer> charFrequencyMap = new HashMap<>();
            for (char currentChar : s.toCharArray()) {
                charFrequencyMap.put(
                        currentChar,
                        charFrequencyMap.getOrDefault(currentChar, 0) + 1);
            }

            int deleteCount = 0;
            for (int frequency : charFrequencyMap.values()) {
                if (frequency % 2 == 1) {
                    deleteCount += frequency - 1;
                } else {
                    deleteCount += frequency - 2;
                }
            }

            return s.length() - deleteCount;
        }
    }

    

}
