package leetcode_grind;

public class Day1069 {
    // https://leetcode.com/problems/check-if-digits-are-equal-in-string-after-operations-i/description/?envType=daily-question&envId=2025-10-23
    static class Solution1 {
        public boolean hasSameDigits(String s) {
            int n = s.length();
            char[] sArray = s.toCharArray();
            for (int i = 1; i <= n - 2; i++) {
                for (int j = 0; j <= n - 1 - i; j++) {
                    int digit1 = sArray[j] - '0';
                    int digit2 = sArray[j + 1] - '0';
                    sArray[j] = (char) (((digit1 + digit2) % 10) + '0');
                }
            }
            return sArray[0] == sArray[1];
        }
    }
}
