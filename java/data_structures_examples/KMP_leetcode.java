package data_structures_examples;

public class KMP_leetcode {
    // https://leetcode.com/problems/rotate-string/description/?envType=daily-question&envId=2024-11-03
    static class Solution1 {
        public boolean rotateString(String s, String goal) {
            if (s.length() != goal.length())
                return false;
            String doubledString = s + s;
            return kmpSearch(doubledString, goal);
        }

        boolean kmpSearch(String text, String pattern) {
            // int[] lps = computeLPS(pattern + '$' + text);
            // for (var n : lps) {
            // if (n == pattern.length()) return true;
            // }
            int[] lps = computeLPS(pattern);
            int textIndex = 0, patternIndex = 0;
            int textLength = text.length(), patternLength = pattern.length();

            while (textIndex < textLength) {
                if (text.charAt(textIndex) == pattern.charAt(patternIndex)) {
                    textIndex++;
                    patternIndex++;
                    if (patternIndex == patternLength)
                        return true;
                } else if (patternIndex > 0) {
                    patternIndex = lps[patternIndex - 1];
                } else {
                    textIndex++;
                }
            }
            return false;
        }

        int[] computeLPS(String pattern) {
            int patternLength = pattern.length();
            int[] lps = new int[patternLength];
            int length = 0, index = 1;

            while (index < patternLength) {
                if (pattern.charAt(index) == pattern.charAt(length)) {
                    length++;
                    lps[index++] = length;
                } else if (length > 0) {
                    length = lps[length - 1];
                } else {
                    lps[index++] = 0;
                }
            }

            return lps;
        }
    }

}
