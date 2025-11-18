package leetcode_grind;

public class Day1095 {
    // https://leetcode.com/problems/1-bit-and-2-bit-characters/description/?envType=daily-question&envId=2025-11-18
    static class Solution1 {
        public boolean isOneBitCharacter(int[] bits) {
            int i = 0;
            while (i < bits.length - 1) {
                i += bits[i] + 1;
            }
            return i == bits.length - 1;
        }
    }

    static class Solution2 {
        public boolean isOneBitCharacter(int[] bits) {
            int i = bits.length - 2;
            while (i >= 0 && bits[i] > 0) {
                i--;
            }
            return (bits.length - i) % 2 == 0;
        }
    }
}
