package leetcode_grind;

public class Day1077 {
    // https://leetcode.com/problems/the-two-sneaky-numbers-of-digitville/description/?envType=daily-question&envId=2025-10-31
    static class Solution1 {
        public int[] getSneakyNumbers(int[] nums) {
            int n = nums.length - 2;
            int y = 0;
            for (int x : nums) {
                y ^= x;
            }
            for (int i = 0; i < n; i++) {
                y ^= i;
            }

            int lowBit = y & -y;
            int x1 = 0;
            int x2 = 0;

            for (int x : nums) {
                if ((x & lowBit) != 0) {
                    x1 ^= x;
                } else {
                    x2 ^= x;
                }
            }

            for (int i = 0; i < n; i++) {
                if ((i & lowBit) != 0) {
                    x1 ^= i;
                } else {
                    x2 ^= i;
                }
            }
            return new int[] { x1, x2 };
        }
    }

}
