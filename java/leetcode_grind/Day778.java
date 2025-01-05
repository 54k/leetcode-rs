package leetcode_grind;

public class Day778 {
    // https://leetcode.com/problems/shifting-letters/description/
    static class Solution1 {
        public String shiftingLetters(String s, int[] shifts) {
            StringBuilder ans = new StringBuilder();
            int X = 0;
            for (int shift : shifts) {
                X = (X + shift) % 26;
            }
            for (int i = 0; i < s.length(); i++) {
                int index = s.charAt(i) - 'a';
                ans.append((char) ((index + X) % 26 + 97));
                X = Math.floorMod(X - shifts[i], 26);
            }
            return ans.toString();
        }
    }
}