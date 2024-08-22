package leetcode_grind;

public class Day644 {
    // https://leetcode.com/problems/number-complement/description/?envType=daily-question&envId=2024-08-22
    static class Solution1 {
        public int findComplement(int num) {
            int n = (int) (Math.log(num) / Math.log(2)) + 1;
            int bitmask = (1 << n) - 1;
            return num ^ bitmask;
        }
    }

    static class Solution2 {
        public int findComplement(int num) {
            return (Integer.highestOneBit(num) << 1) - num - 1;
        }
    }
}
