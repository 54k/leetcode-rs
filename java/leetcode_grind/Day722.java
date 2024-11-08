package leetcode_grind;

public class Day722 {
    // https://leetcode.com/problems/maximum-xor-for-each-query/description/?envType=daily-question&envId=2024-11-08
    static class Solution1 {
        public int[] getMaximumXor(int[] nums, int maximumBit) {
            int xorProduct = 0;
            for (int i = 0; i < nums.length; i++) {
                xorProduct = xorProduct ^ nums[i];
            }
            int[] ans = new int[nums.length];
            int mask = (1 << maximumBit) - 1;
            for (int i = 0; i < nums.length; i++) {
                ans[i] = xorProduct ^ mask;
                xorProduct = xorProduct ^ nums[nums.length - 1 - i];
            }
            return ans;
        }
    }
}
