package leetcode_grind;

public class Day774 {
    // https://leetcode.com/problems/maximum-score-after-splitting-a-string/description/?envType=daily-question&envId=2025-01-01
    static class Solution1 {
        public int maxScore(String s) {
            int ones = 0;
            int zeros = 0;
            int best = Integer.MIN_VALUE;

            for (int i = 0; i < s.length() - 1; i++) {
                if (s.charAt(i) == '1') {
                    ones++;
                } else {
                    zeros++;
                }

                best = Math.max(best, zeros - ones);
            }

            if (s.charAt(s.length() - 1) == '1') {
                ones++;
            }

            return best + ones;
        }
    }
}
