package leetcode_grind;

import java.util.HashSet;

public class Day976 {
    // https://leetcode.com/problems/maximum-erasure-value/description/?envType=daily-question&envId=2025-07-22
    static class Solution1 {
        public int maximumUniqueSubarray(int[] nums) {
            int result = 0;
            int currentSum = 0;
            HashSet<Integer> set = new HashSet<>();
            int start = 0;
            for (int end = 0; end < nums.length; end++) {
                while (set.contains(nums[end])) {
                    set.remove(nums[start]);
                    currentSum -= nums[start];
                    start++;
                }
                currentSum += nums[end];
                set.add(nums[end]);
                result = Math.max(result, currentSum);
            }
            return result;
        }
    }

}
