package leetcode_grind;

import java.util.Arrays;

public class Day912 {
    // https://leetcode.com/problems/type-of-triangle/description/?envType=daily-question&envId=2025-05-19
    static class Solution1 {
        public String triangleType(int[] nums) {
            Arrays.sort(nums);
            if (nums[0] + nums[1] <= nums[2]) {
                return "none";
            } else if (nums[0] == nums[2]) {
                return "equilateral";
            } else if (nums[0] == nums[1] || nums[1] == nums[2]) {
                return "isosceles";
            } else {
                return "scalene";
            }
        }
    }
}
