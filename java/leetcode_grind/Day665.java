package leetcode_grind;

public class Day665 {
    // https://leetcode.com/problems/count-the-number-of-consistent-strings/description/?envType=daily-question&envId=2024-09-12
    static class Solution1 {
        public int countConsistentStrings(String allowed, String[] words) {
            int allowedBits = 0;
            for (int i = 0; i < allowed.length(); i++) {
                allowedBits |= 1 << (allowed.charAt(i) - 'a');
            }

            int consistentCount = 0;
            for (String word : words) {
                boolean isConsistent = true;

                for (int i = 0; i < word.length(); i++) {
                    int bit = (allowedBits >> (word.charAt(i) - 'a')) & 1;
                    if (bit == 0) {
                        isConsistent = false;
                    }
                }

                if (isConsistent) {
                    consistentCount++;
                }
            }
            return consistentCount;
        }
    }
}
