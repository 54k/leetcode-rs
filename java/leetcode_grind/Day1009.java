package leetcode_grind;

public class Day1009 {
    // https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/description/?envType=daily-question&envId=2025-08-24
    static class Solution1 {
        public int longestSubarray(int[] nums) {
            int zeroCount = 0;
            int longestWindow = 0;
            int start = 0;
            for (int i = 0; i < nums.length; i++) {
                zeroCount += (nums[i] == 0 ? 1 : 0);
                while (zeroCount > 1) {
                    zeroCount -= (nums[start] == 0 ? 1 : 0);
                    start++;
                }

                longestWindow = Math.max(longestWindow, i - start);
            }
            return longestWindow;
        }
    }
}
