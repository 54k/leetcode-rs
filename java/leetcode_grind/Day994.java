package leetcode_grind;

public class Day994 {
    // https://leetcode.com/problems/power-of-two/description/?envType=daily-question&envId=2025-08-09
    static class Solution1 {
        public boolean isPowerOfTwo(int n) {
            if (n <= 0)
                return false;
            return (n & (n - 1)) == 0;
        }
    }
}
