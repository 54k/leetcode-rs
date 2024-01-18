package leetcode_grind;

public class Day432 {
    // https://leetcode.com/problems/climbing-stairs/description
    static class Solution1 {
        public int climbStairs(int n) {
            int a = 1;
            int b = 1;
            for (int i = 2; i <= n; i++) {
                int c = a + b;
                a = b;
                b = c;
            }
            return b;
        }
    }
}
