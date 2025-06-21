package leetcode_grind;

import java.util.Arrays;

public class Day942 {
    // https://leetcode.com/problems/divide-array-into-arrays-with-max-difference/description/?envType=daily-question&envId=2025-06-21
    static class Solution1 {
        public int[][] divideArray(int[] nums, int k) {
            Arrays.sort(nums);
            int[][] ans = new int[nums.length / 3][3];
            for (int i = 0; i < nums.length; i += 3) {
                if (nums[i + 2] - nums[i] > k) {
                    return new int[0][0];
                }
                ans[i / 3] = new int[] { nums[i], nums[i + 1], nums[i + 2] };
            }
            return ans;
        }
    }
}
