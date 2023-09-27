package leetcode_grind;

public class Day319 {
    // https://leetcode.com/problems/decoded-string-at-index/description/
    static class Solution {
        public String decodeAtIndex(String s, int k) {
            long size = 0;
            var n = s.length();

            for (var i = 0; i < n; i++) {
                var c = s.charAt(i);
                if (Character.isDigit(c)) {
                    size *= c - '0';
                } else {
                    size++;
                }
            }

            for (var i = n - 1; i >= 0; i--) {
                var c = s.charAt(i);
                k %= size;
                if (k == 0 && Character.isLetter(c)) {
                    return Character.toString(c);
                }

                if (Character.isDigit(c)) {
                    size /= c - '0';
                } else {
                    size--;
                }
            }

            return "";
        }
    }
}
