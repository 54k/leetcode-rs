package leetcode_grind;

public class Day805 {
    // https://leetcode.com/problems/special-array-i/description/?envType=daily-question&envId=2025-02-01
    static class Solution1 {
        public boolean isArraySpecial(int[] nums) {
            int m = nums.length;
            for (int i = 0; i < m - 1; i++) {
                if (nums[i] % 2 == nums[i + 1] % 2) {
                    return false;
                }
            }
            return true;
        }
    }
}
