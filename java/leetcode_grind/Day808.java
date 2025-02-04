package leetcode_grind;

public class Day808 {
    // https://leetcode.com/problems/maximum-ascending-subarray-sum/description/?envType=daily-question&envId=2025-02-04
    static class Solution1 {
        public int maxAscendingSum(int[] nums) {
            var ans = 0;
            var cur = nums[0];
            for (int i = 1; i < nums.length; i++) {
                if (nums[i] > nums[i - 1]) {
                    cur += nums[i];
                } else {
                    ans = Math.max(ans, Math.max(nums[i], cur));
                    cur = nums[i];
                }
            }
            return Math.max(ans, cur);
        }
    }
}
