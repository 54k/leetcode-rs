package leetcode_grind;

public class Day529 {
    // https://leetcode.com/problems/n-th-tribonacci-number/description
    static class Solution1 {
        public int tribonacci(int n) {
            if (n == 0)
                return 0;
            int a = 0, b = 1, c = 1;
            for (int i = 3; i <= n; i++) {
                int d = a + b + c;
                a = b;
                b = c;
                c = d;
            }
            return c;
        }
    }
}
