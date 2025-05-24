package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day917 {
    // https://leetcode.com/problems/find-words-containing-character/description/?envType=daily-question&envId=2025-05-24
    static class Solution1 {
        public List<Integer> findWordsContaining(String[] words, char x) {
            List<Integer> res = new ArrayList<>();
            int n = words.length;
            for (int i = 0; i < n; i++) {
                if (words[i].indexOf(x) != -1) {
                    res.add(i);
                }
            }
            return res;
        }
    }
}
