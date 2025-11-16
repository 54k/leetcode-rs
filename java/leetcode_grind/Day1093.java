package leetcode_grind;

public class Day1093 {
    // https://leetcode.com/problems/number-of-substrings-with-only-1s/description/?envType=daily-question&envId=2025-11-16
    static class Solution1 {
        public int numSub(String s) {
            final int MODULO = 1000_000_007;
            long total = 0;
            int length = s.length();
            long consecutive = 0;

            for (int i = 0; i < length; i++) {
                char c = s.charAt(i);
                if (c == '0') {
                    total += (consecutive * (consecutive + 1)) / 2;
                    total %= MODULO;
                    consecutive = 0;
                } else {
                    consecutive++;
                }
            }
            total += (consecutive * (consecutive + 1)) / 2;
            total %= MODULO;
            return (int) total;
        }
    }
}
