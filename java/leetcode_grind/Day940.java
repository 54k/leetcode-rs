package leetcode_grind;

public class Day940 {
    // https://leetcode.com/problems/maximum-difference-between-increasing-elements/description/?envType=daily-question&envId=2025-06-16
    static class Solution1 {
        public int maximumDifference(int[] nums) {
            int n = nums.length;
            int ans = -1, premin = nums[0];
            for (int i = 1; i < n; i++) {
                if (nums[i] > premin) {
                    ans = Math.max(ans, nums[i] - premin);
                } else {
                    premin = nums[i];
                }
            }
            return ans;
        }
    }
}
