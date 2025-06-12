package leetcode_grind;

public class Day936 {
    // https://leetcode.com/problems/maximum-difference-between-adjacent-elements-in-a-circular-array/description/?envType=daily-question&envId=2025-06-12
    static class Solution1 {
        public int maxAdjacentDistance(int[] nums) {
            int n = nums.length;
            int res = Math.abs(nums[0] - nums[n - 1]);
            for (int i = 0; i < n - 1; i++) {
                res = Math.max(res, Math.abs(nums[i] - nums[i + 1]));
            }
            return res;
        }
    }
}
