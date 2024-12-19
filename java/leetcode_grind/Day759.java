package leetcode_grind;

public class Day759 {
    // https://leetcode.com/problems/construct-string-with-repeat-limit/description/?envType=daily-question&envId=2024-12-17
    static class Solution1 {
        public String repeatLimitedString(String s, int repeatLimit) {
            var freq = new int[26];
            for (var ch : s.toCharArray()) {
                freq[ch - 'a']++;
            }
            var result = new StringBuilder();
            int currentCharIndex = 25;
            while (currentCharIndex >= 0) {
                if (freq[currentCharIndex] == 0) {
                    currentCharIndex--;
                    continue;
                }

                int use = Math.min(freq[currentCharIndex], repeatLimit);
                for (int k = 0; k < use; k++) {
                    result.append((char) ('a' + currentCharIndex));
                }
                freq[currentCharIndex] -= use;

                if (freq[currentCharIndex] > 0) {
                    int smallerCharIndex = currentCharIndex - 1;
                    while (smallerCharIndex >= 0 && freq[smallerCharIndex] == 0) {
                        smallerCharIndex--;
                    }
                    if (smallerCharIndex < 0) {
                        break;
                    }
                    result.append((char) ('a' + smallerCharIndex));
                    freq[smallerCharIndex]--;
                }
            }
            return result.toString();
        }
    }

}
