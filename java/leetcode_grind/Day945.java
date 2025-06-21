package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day945 {
    // https://leetcode.com/problems/minimum-deletions-to-make-string-k-special/description/?envType=daily-question&envId=2025-06-21
    static class Solution1 {
        public int minimumDeletions(String word, int k) {
            Map<Character, Integer> cnt = new HashMap<>();
            for (char ch : word.toCharArray()) {
                cnt.put(ch, cnt.getOrDefault(ch, 0) + 1);
            }
            int res = word.length();
            for (int a : cnt.values()) {
                int deleted = 0;
                for (int b : cnt.values()) {
                    if (a > b) {
                        deleted += b;
                    } else if (b > a + k) {
                        deleted += b - (a + k);
                    }
                }
                res = Math.min(res, deleted);
            }
            return res;
        }
    }

}
