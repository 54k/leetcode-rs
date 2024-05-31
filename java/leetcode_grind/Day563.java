package leetcode_grind;

public class Day563 {
    // https://leetcode.com/problems/single-number-iii/
    static class Solution1 {
        public int[] singleNumber(int[] nums) {
            int bitmask = 0;
            for (int num : nums)
                bitmask ^= num;

            int diff = bitmask & (-bitmask);
            int x = 0;
            for (int num : nums) {
                if ((num & diff) != 0)
                    x ^= num;
            }
            return new int[] { x, bitmask ^ x };
        }
    }
}
