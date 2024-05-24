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

    // https://leetcode.com/problems/next-permutation/description/
    static class Solution2 {
        public void nextPermutation(int[] nums) {
            int i = nums.length - 2;
            while (i >= 0 && nums[i + 1] <= nums[i]) {
                i--;
            }
            if (i >= 0) {
                int j = nums.length - 1;
                while (nums[j] <= nums[i]) {
                    j--;
                }
                swap(nums, i, j);
            }
            reverse(nums, i + 1);
        }

        private void reverse(int[] nums, int start) {
            int i = start, j = nums.length - 1;
            while (i < j) {
                swap(nums, i, j);
                i++;
                j--;
            }
        }

        private void swap(int[] nums, int i, int j) {
            int temp = nums[i];
            nums[i] = nums[j];
            nums[j] = temp;
        }
    }

    // https://leetcode.com/problems/longest-continuous-increasing-subsequence/description/
    static class Solution3 {
        public int findLengthOfLCIS(int[] nums) {
            int ans = 0;
            int anchor = 0;
            for (int i = 0; i < nums.length; i++) {
                if (i > 0 && nums[i - 1] >= nums[i])
                    anchor = i;
                ans = Math.max(ans, i - anchor + 1);
            }
            return ans;
        }
    }
}
