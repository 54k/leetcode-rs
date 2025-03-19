package leetcode_grind;

public class Day851 {
    // https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i/description/?envType=daily-question&envId=2025-03-19
    static class Solution1 {
        public int minOperations(int[] nums) {
            int ops = 0;
            for (int i = 0; i < nums.length; i++) {
                if (nums[i] == 0) {
                    if (i < nums.length - 2) {
                        nums[i] ^= 1;
                        nums[i + 1] ^= 1;
                        nums[i + 2] ^= 1;
                        ops++;
                    } else {
                        return -1;
                    }
                }
            }
            return ops;
        }
    }
}
