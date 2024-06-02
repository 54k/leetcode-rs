package leetcode_grind;

public class Day565 {
    // https://leetcode.com/problems/reverse-string/description
    static class Solution1 {
        public void reverseString(char[] s) {
            for (int i = 0; i < s.length / 2; i++) {
                char t = s[i];
                s[i] = s[s.length - i - 1];
                s[s.length - i - 1] = t;
            }
        }
    }
}
