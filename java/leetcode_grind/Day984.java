package leetcode_grind;

public class Day984 {
    // https://leetcode.com/problems/longest-subarray-with-maximum-bitwise-and/description/?envType=daily-question&envId=2025-07-30
    static class Solution1 {
        public int longestSubarray(int[] nums) {
            int maxVal = 0, ans = 0, currentStreak = 0;
            for (int num : nums) {
                if (maxVal < num) {
                    maxVal = num;
                    ans = currentStreak = 0;
                }

                if (maxVal == num) {
                    currentStreak++;
                } else {
                    currentStreak = 0;
                }

                ans = Math.max(ans, currentStreak);
            }
            return ans;
        }
    }
}
