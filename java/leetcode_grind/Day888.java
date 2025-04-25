package leetcode_grind;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day888 {
    // https://leetcode.com/problems/count-of-interesting-subarrays/description/?envType=daily-question&envId=2025-04-25
    static class Solution1 {
        public long countInterestingSubarrays(List<Integer> nums, int modulo, int k) {
            int n = nums.size();
            Map<Integer, Integer> cnt = new HashMap<>();
            long res = 0;
            int prefix = 0;
            cnt.put(0, 1);

            for (int i = 0; i < n; i++) {
                prefix += nums.get(i) % modulo == k ? 1 : 0;
                res += cnt.getOrDefault((prefix - k + modulo) % modulo, 0);
                cnt.put(prefix % modulo, cnt.getOrDefault(prefix % modulo, 0) + 1);
            }
            return res;
        }
    }
}
