package leetcode_grind;

public class Day1021 {
    // https://leetcode.com/problems/minimum-operations-to-make-the-integer-zero/description/?envType=daily-question&envId=2025-09-05
    static class Solution1 {
        public int makeTheIntegerZero(int num1, int num2) {
            int k = 1;
            while (true) {
                long x = num1 - (long) num2 * k;
                if (x < k) {
                    return -1;
                }
                if (k >= Long.bitCount(x)) {
                    return k;
                }
                k++;
            }
        }
    }
}
