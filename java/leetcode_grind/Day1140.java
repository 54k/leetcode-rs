package leetcode_grind;

public class Day1140 {
    // https://leetcode.com/problems/n-repeated-element-in-size-2n-array/description/?envType=daily-question&envId=2026-01-02
    static class Solution1 {
        public int repeatedNTimes(int[] nums) {
            for (int k = 1; k <= 3; k++) {
                for (int i = 0; i < nums.length - k; ++i) {
                    if (nums[i] == nums[i + k]) {
                        return nums[i];
                    }
                }
            }
            throw null;
        }
    }
}
