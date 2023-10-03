package leetcode_grind;

import java.util.HashMap;

public class Day325 {
    // https://leetcode.com/problems/number-of-good-pairs/description
    static class Solution {
        public int numIdenticalPairs(int[] nums) {
            var m = new HashMap<Integer, Integer>();
            var ans = 0;
            for (var n : nums) {
                ans += m.getOrDefault(n, 0);
                m.put(n, m.getOrDefault(n, 0) + 1);
            }
            return ans;
        }
    }
}
