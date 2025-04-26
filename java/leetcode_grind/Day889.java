package leetcode_grind;

public class Day889 {
    // https://leetcode.com/problems/count-subarrays-with-fixed-bounds/description/?envType=daily-question&envId=2025-04-26
    static class Solution1 {
        public long countSubarrays(int[] nums, int minK, int maxK) {
            long answer = 0;
            int minPosition = -1, maxPosition = -1, leftBound = -1;

            for (int i = 0; i < nums.length; i++) {
                if (nums[i] < minK || nums[i] > maxK) {
                    leftBound = i;
                }
                if (nums[i] == minK) {
                    minPosition = i;
                }
                if (nums[i] == maxK) {
                    maxPosition = i;
                }
                answer += Math.max(0, Math.min(maxPosition, minPosition) - leftBound);
            }
            return answer;
        }
    }
}
