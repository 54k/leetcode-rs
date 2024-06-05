package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day568 {
    // https://leetcode.com/problems/find-common-characters/description
    static class Solution1 {
        public List<String> commonChars(String[] words) {
            int wordsSize = words.length;
            int[] commonCharacterCounts = new int[26];
            int[] currentCharacterCounts = new int[26];
            List<String> result = new ArrayList<>();

            for (char ch : words[0].toCharArray()) {
                commonCharacterCounts[ch - 'a']++;
            }

            for (int i = 1; i < wordsSize; i++) {
                Arrays.fill(currentCharacterCounts, 0);

                for (char ch : words[i].toCharArray()) {
                    currentCharacterCounts[ch - 'a']++;
                }

                for (int letter = 0; letter < 26; letter++) {
                    commonCharacterCounts[letter] = Math.min(
                            commonCharacterCounts[letter],
                            currentCharacterCounts[letter]);
                }
            }

            for (int letter = 0; letter < 26; letter++) {
                for (int commonCount = 0; commonCount < commonCharacterCounts[letter]; commonCount++) {
                    result.add(String.valueOf((char) (letter + 'a')));
                }
            }
            return result;
        }
    }

    // https://leetcode.com/problems/longest-common-subsequence-between-sorted-arrays/description
    static class Solution2 {
        public List<Integer> longestCommonSubsequence(int[][] arrays) {
            Map<Integer, Integer> frequencies = new HashMap<>();
            List<Integer> longestCommonSubseq = new ArrayList<>();

            for (int[] array : arrays) {
                for (int num : array) {
                    frequencies.put(num, frequencies.getOrDefault(num, 0) + 1);
                    if (frequencies.get(num) == arrays.length) {
                        longestCommonSubseq.add(num);
                    }
                }
            }
            return longestCommonSubseq;
        }
    }
}
