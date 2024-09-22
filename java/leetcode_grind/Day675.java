package leetcode_grind;

import java.util.*;

public class Day675 {
    // https://leetcode.com/problems/longest-word-with-all-prefixes/description/?envType=weekly-question&envId=2024-09-22
    static class Solution1 {
        public String longestWord(String[] words) {
            Arrays.sort(words);
            Set<String> validWords = new HashSet<>();
            String longestValidWord = "";

            for (String currentWord : words) {
                if (currentWord.length() == 1
                        || validWords.contains(currentWord.substring(0, currentWord.length() - 1))) {
                    validWords.add(currentWord);

                    if (longestValidWord.length() < currentWord.length()) {
                        longestValidWord = currentWord;
                    }
                }
            }

            return longestValidWord;
        }
    }
}
