package leetcode_grind;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;

public class Day979 {
    // https://leetcode.com/problems/maximum-unique-subarray-sum-after-deletion/description/?envType=daily-question&envId=2025-07-25
    static class Solution1 {
        public int maxSum(int[] nums) {
            Set<Integer> positiveNumsSet = new HashSet<>();
            for (int num : nums) {
                if (num > 0) {
                    positiveNumsSet.add(num);
                }
            }
            if (positiveNumsSet.isEmpty()) {
                return Arrays.stream(nums).max().getAsInt();
            }
            return positiveNumsSet.stream().mapToInt(Integer::intValue).sum();
        }
    }
}
