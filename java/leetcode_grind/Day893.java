package leetcode_grind;

public class Day893 {
    // https://leetcode.com/problems/find-numbers-with-even-number-of-digits/description/?envType=daily-question&envId=2025-04-30
    static class Solution1 {
        public int findNumbers(int[] nums) {
            int evenDigitCount = 0;
            for (int num : nums) {
                int digitCount = (int) Math.floor(Math.log10(num)) + 1;
                if (digitCount % 2 == 0) {
                    evenDigitCount++;
                }
            }
            return evenDigitCount;
        }
    }
}
