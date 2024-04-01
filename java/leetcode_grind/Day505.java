package leetcode_grind;

public class Day505 {
    // https://leetcode.com/problems/length-of-last-word/description/
    static class Solution1 {
        public int lengthOfLastWord(String s) {
            int i = s.length() - 1;
            int ans = 0;
            while (i >= 0) {
                if (s.charAt(i--) != ' ') {
                    ans++;
                } else if (ans > 0) {
                    break;
                }
            }
            return ans;
        }
    }
}
