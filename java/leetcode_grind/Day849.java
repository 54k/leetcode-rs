package leetcode_grind;

import java.util.Arrays;

public class Day849 {
    // https://leetcode.com/problems/divide-array-into-equal-pairs/description/?envType=daily-question&envId=2025-03-17
    static class Solution1 {
        public boolean divideArray(int[] nums) {
            Arrays.sort(nums);
            for (int pos = 0; pos < nums.length; pos += 2) {
                if (nums[pos] != nums[pos + 1]) {
                    return false;
                }
            }
            return true;
        }
    }

}
