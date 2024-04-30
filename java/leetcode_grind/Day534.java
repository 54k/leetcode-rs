package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day534 {

    // https://leetcode.com/problems/number-of-wonderful-substrings/description
    static class Solution {
        public long wonderfulSubstrings(String word) {
            int N = word.length();
            Map<Integer, Integer> freq = new HashMap<>();
            freq.put(0, 1);
            int mask = 0;
            long res = 0L;

            for (int i = 0; i < N; i++) {
                char c = word.charAt(i);
                int bit = c - 'a';
                mask ^= (1 << bit);
                res += freq.getOrDefault(mask, 0);
                freq.put(mask, freq.getOrDefault(mask, 0) + 1);

                for (int odd_c = 0; odd_c < 10; odd_c++) {
                    res += freq.getOrDefault(mask ^ (1 << odd_c), 0);
                }
            }
            return res;
        }
    }
}