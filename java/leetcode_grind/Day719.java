package leetcode_grind;

public class Day719 {
    // https://leetcode.com/problems/minimum-number-of-changes-to-make-binary-string-beautiful/description/?envType=daily-question&envId=2024-11-05
    static class Solution1 {
        public int minChanges(String s) {
            char currentChar = s.charAt(0);
            int consecutiveCount = 0;
            int minChangesRequired = 0;

            for (int i = 0; i < s.length(); i++) {
                if (s.charAt(i) == currentChar) {
                    consecutiveCount++;
                    continue;
                }
                if (consecutiveCount % 2 == 0) {
                    consecutiveCount = 1;
                } else {
                    consecutiveCount = 0;
                    minChangesRequired++;
                }
                currentChar = s.charAt(i);
            }
            return minChangesRequired;
        }
    }
}
