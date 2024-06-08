package leetcode_grind;

import java.util.HashMap;

public class Day571 {
    // https://leetcode.com/problems/continuous-subarray-sum/description
    static class Solution1 {
        public boolean checkSubarraySum(int[] nums, int k) {
            int prefixMod = 0;
            HashMap<Integer, Integer> modSeen = new HashMap<>();
            modSeen.put(0, -1);

            for (int i = 0; i < nums.length; i++) {
                prefixMod = (prefixMod + nums[i]) % k;

                if (modSeen.containsKey(prefixMod)) {
                    if (i - modSeen.get(prefixMod) > 1) {
                        return true;
                    }
                } else {
                    modSeen.put(prefixMod, i);
                }
            }

            return false;
        }
    }
}
