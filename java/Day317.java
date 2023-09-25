public class Day317 {
    // https://leetcode.com/problems/find-the-difference/description
    static class Solution {
        public char findTheDifference(String s, String t) {
            char ch = 0;
            for (var c : s.toCharArray()) {
                ch ^= c;
            }
            for (var c : t.toCharArray()) {
                ch ^= c;
            }
            return ch;
        }
    }
}
