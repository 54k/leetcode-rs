package leetcode_grind;

public class Day718 {
    // https://leetcode.com/problems/string-compression-iii/description/?envType=daily-question&envId=2024-11-04
    static class Solution1 {
        public String compressedString(String word) {
            StringBuilder comp = new StringBuilder("");
            int pos = 0;
            while (pos < word.length()) {
                int consecutiveCount = 0;
                char currentChar = word.charAt(pos);
                while (pos < word.length() && consecutiveCount < 9 && word.charAt(pos) == currentChar) {
                    consecutiveCount++;
                    pos++;
                }
                comp.append(consecutiveCount).append(currentChar);
            }
            return comp.toString();
        }
    }
}
