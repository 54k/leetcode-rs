package leetcode_grind;

public class Day566 {
    // https://leetcode.com/problems/append-characters-to-string-to-make-subsequence/description
    static class Solution {
        public int appendCharacters(String s, String t) {
            int first = 0, longestPrefix = 0;
            while (first < s.length() && longestPrefix < t.length()) {
                if (s.charAt(first) == t.charAt(longestPrefix)) {
                    longestPrefix++;
                }
                first++;
            }
            return t.length() - longestPrefix;
        }
    }
}
