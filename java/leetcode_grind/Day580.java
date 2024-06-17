package leetcode_grind;

public class Day580 {
    // https://leetcode.com/problems/sum-of-square-numbers/description/
    static class Solution1 {
        public boolean judgeSquareSum(int c) {
            for (int i = 2; i * i <= c; i++) {
                int count = 0;
                if (c % i == 0) {
                    while (c % i == 0) {
                        count++;
                        c /= i;
                    }
                    if (i % 4 == 3 && count % 2 != 0) {
                        return false;
                    }
                }
            }
            return c % 4 != 3;
        }
    }
}
