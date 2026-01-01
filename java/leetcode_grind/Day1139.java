package leetcode_grind;

public class Day1139 {
    // https://leetcode.com/problems/plus-one/description/?envType=daily-question&envId=2026-01-01
    static class Solution1 {
        public int[] plusOne(int[] digits) {
            for (int i = digits.length - 1; i >= 0; i--) {
                if (digits[i] == 9) {
                    digits[i] = 0;
                } else {
                    digits[i] += 1;
                    return digits;
                }
            }

            var ans = new int[digits.length + 1];
            ans[0] = 1;
            return ans;
        }
    }
}
