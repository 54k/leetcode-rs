package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day842 {
    // https://leetcode.com/problems/count-of-substrings-containing-every-vowel-and-k-consonants-ii/description/?envType=daily-question&envId=2025-03-10
    static class Solution1 {
        public long countOfSubstrings(String word, int k) {
            long numValidSubstrings = 0;
            int start = 0;
            int end = 0;

            Map<Character, Integer> vowelCount = new HashMap<>();
            int consonantCount = 0;

            int[] nextConsonant = new int[word.length()];
            int nextConsonantIndex = word.length();
            for (int i = word.length() - 1; i >= 0; i--) {
                nextConsonant[i] = nextConsonantIndex;
                if (!isVowel(word.charAt(i))) {
                    nextConsonantIndex = i;
                }
            }

            while (end < word.length()) {
                char newLetter = word.charAt(end);
                if (isVowel(newLetter)) {
                    vowelCount.put(newLetter, vowelCount.getOrDefault(newLetter, 0) + 1);
                } else {
                    consonantCount++;
                }

                while (consonantCount > k) {
                    char startLetter = word.charAt(start);
                    if (isVowel(startLetter)) {
                        vowelCount.put(startLetter, vowelCount.get(startLetter) - 1);
                        if (vowelCount.get(startLetter) == 0) {
                            vowelCount.remove(startLetter);
                        }
                    } else {
                        consonantCount--;
                    }
                    start++;
                }

                while (start < word.length() && vowelCount.keySet().size() == 5 && consonantCount == k) {
                    numValidSubstrings += nextConsonant[end] - end;
                    char startLetter = word.charAt(start);
                    if (isVowel(startLetter)) {
                        vowelCount.put(startLetter, vowelCount.get(startLetter) - 1);
                        if (vowelCount.get(startLetter) == 0) {
                            vowelCount.remove(startLetter);
                        }
                    } else {
                        consonantCount--;
                    }

                    start++;
                }

                end++;
            }
            return numValidSubstrings;
        }

        boolean isVowel(char c) {
            return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
        }
    }

    static class Solution2 {
        public long countOfSubstrings(String word, int k) {
            return atLeastK(word, k) - atLeastK(word, k + 1);
        }

        long atLeastK(String word, int k) {
            long numValidSubstrings = 0;
            int start = 0;
            int end = 0;

            HashMap<Character, Integer> vowelCount = new HashMap<>();
            int consonantCount = 0;

            while (end < word.length()) {
                char newLetter = word.charAt(end);

                if (isVowel(newLetter)) {
                    vowelCount.put(newLetter, vowelCount.getOrDefault(newLetter, 0) + 1);
                } else {
                    consonantCount++;
                }

                while (vowelCount.size() == 5 && consonantCount >= k) {
                    numValidSubstrings += word.length() - end;
                    char startLetter = word.charAt(start);
                    if (isVowel(startLetter)) {
                        vowelCount.put(startLetter, vowelCount.get(startLetter) - 1);
                        if (vowelCount.get(startLetter) == 0) {
                            vowelCount.remove(startLetter);
                        }
                    } else {
                        consonantCount--;
                    }
                    start++;
                }

                end++;
            }

            return numValidSubstrings;
        }

        boolean isVowel(char c) {
            return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
        }
    }

}
