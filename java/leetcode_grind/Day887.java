package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;

public class Day887 {
    
    // https://leetcode.com/problems/count-complete-subarrays-in-an-array/description/?envType=daily-question&envId=2025-04-24
    static class Solution1 {
        public int countCompleteSubarrays(int[] nums) {
            int res = 0;
            Map<Integer, Integer> cnt = new HashMap<>();
            int n = nums.length;
            int right = 0;
            int distinct = new HashSet<>(Arrays.stream(nums).boxed().toList()).size();

            for (int left = 0; left < n; left++) {
                if (left > 0) {
                    int remove = nums[left - 1];
                    cnt.put(remove, cnt.get(remove) - 1);
                    if (cnt.get(remove) == 0) {
                        cnt.remove(remove);
                    }
                }

                while (right < n && cnt.size() < distinct) {
                    int add = nums[right];
                    cnt.put(add, cnt.getOrDefault(add, 0) + 1);
                    right++;
                }

                if (cnt.size() == distinct) {
                    res += (n - right + 1);
                }
            }
            return res;
        }
    }
}
