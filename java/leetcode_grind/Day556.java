package leetcode_grind;

import java.util.Arrays;

public class Day556 {
    // https://leetcode.com/problems/maximum-score-words-formed-by-letters/description
    static class Solution1 {
        public int maxScoreWords(String[] words, char[] letters, int[] score) {
            int W = words.length;
            int[] freq = new int[26];
            for (char c : letters) {
                freq[c - 'a']++;
            }
            int maxScore = 0;
            int[] subsetLetters = new int[26];
            for (int mask = 0; mask < 1 << W; mask++) {
                Arrays.fill(subsetLetters, 0);
                for (int i = 0; i < W; i++) {
                    if ((mask & (1 << i)) > 0) {
                        int L = words[i].length();
                        for (int j = 0; j < L; j++) {
                            subsetLetters[words[i].charAt(j) - 'a']++;
                        }
                    }
                }
                maxScore = Math.max(maxScore, subsetScore(subsetLetters, score, freq));
            }
            return maxScore;
        }

        private int subsetScore(int[] subsetLetters, int[] score, int[] freq) {
            int totalScore = 0;
            for (int c = 0; c < 26; c++) {
                totalScore += subsetLetters[c] * score[c];
                if (subsetLetters[c] > freq[c]) {
                    return 0;
                }
            }
            return totalScore;
        }
    }
}
