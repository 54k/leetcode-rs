package leetcode_grind;

public class Day504 {
    // https://leetcode.com/problems/count-subarrays-with-fixed-bounds/description/
    static class Solution {
        public long countSubarrays(int[] nums, int minK, int maxK) {
            long leftBound = -1;
            long mn = -1;
            long mx = -1;
            long ans = 0;
            for (int i = 0; i < nums.length; i++) {
                if (minK > nums[i] || nums[i] > maxK) {
                    leftBound = i;
                }
                if (nums[i] == minK) {
                    mn = i;
                }
                if (nums[i] == maxK) {
                    mx = i;
                }
                ans += Math.max(0, Math.min(mn, mx) - leftBound);
            }
            return ans;
        }
    }
}
