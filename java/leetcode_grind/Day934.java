package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day934 {
    // https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-i/description/?envType=daily-question&envId=2025-06-10
    static class Solution1 {
        public int maxDifference(String s) {
            Map<Character, Integer> c = new HashMap<>();
            for (char ch : s.toCharArray()) {
                c.put(ch, c.getOrDefault(ch, 0) + 1);
            }
            int maxOdd = 1, minEven = s.length();
            for (int value : c.values()) {
                if (value % 2 == 1) {
                    maxOdd = Math.max(maxOdd, value);
                } else {
                    minEven = Math.min(minEven, value);
                }
            }
            return maxOdd - minEven;
        }
    }
}
