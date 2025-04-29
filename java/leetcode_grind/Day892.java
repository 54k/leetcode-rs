package leetcode_grind;

import java.util.*;

public class Day892 {
    // https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/description/?envType=daily-question&envId=2025-04-29
    static class Solution1 {
        public long countSubarrays(int[] nums, int k) {
            int maxElement = Arrays.stream(nums).max().getAsInt();
            long ans = 0, start = 0;
            int maxElementsInWindow = 0;

            for (int end = 0; end < nums.length; end++) {
                if (nums[end] == maxElement) {
                    maxElementsInWindow++;
                }
                while (maxElementsInWindow == k) {
                    if (nums[(int) start] == maxElement) {
                        maxElementsInWindow--;
                    }
                    start++;
                }
                ans += start;
            }
            return ans;
        }
    }
}
