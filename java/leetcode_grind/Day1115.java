package leetcode_grind;

public class Day1115 {
    // https://leetcode.com/problems/count-square-sum-triples/description/?envType=daily-question&envId=2025-12-08
    static class Solution1 {
        public int countTriples(int n) {
            int res = 0;
            // enumerate a and b
            for (int a = 1; a <= n; ++a) {
                for (int b = 1; b <= n; ++b) {
                    // determine if it meets the requirements
                    int c = (int) Math.sqrt(a * a + b * b + 1.0);
                    if (c <= n && c * c == a * a + b * b) {
                        ++res;
                    }
                }
            }
            return res;
        }
    }
}
