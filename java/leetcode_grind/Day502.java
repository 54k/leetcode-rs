package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Day502 {
    // https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/description/
    static class Solution1 {
        public long countSubarrays(int[] nums, int k) {
            int maxElement = Arrays.stream(nums).max().getAsInt();
            List<Integer> indexesOfMaxELements = new ArrayList<>();
            long ans = 0;

            for (int i = 0; i < nums.length; i++) {
                if (nums[i] == maxElement) {
                    indexesOfMaxELements.add(i);
                }
                int freq = indexesOfMaxELements.size();
                if (freq >= k) {
                    ans += indexesOfMaxELements.get(freq - k) + 1;
                }
            }

            return ans;
        }
    }
}
