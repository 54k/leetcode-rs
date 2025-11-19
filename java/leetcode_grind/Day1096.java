package leetcode_grind;

import java.util.Arrays;

public class Day1096 {
    // https://leetcode.com/problems/keep-multiplying-found-values-by-two/description/?envType=daily-question&envId=2025-11-19
    static class Solution1 {
        public int findFinalValue(int[] nums, int original) {
            Arrays.sort(nums);
            for (int num : nums) {
                if (original == num) {
                    original *= 2;
                }
            }
            return original;
        }
    }
}
