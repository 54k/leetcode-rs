package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day813 {
    // https://leetcode.com/problems/count-number-of-bad-pairs/description/?envType=daily-question&envId=2025-02-09
    static class Solution1 {
        public long countBadPairs(int[] nums) {
            long badPairs = 0;
            Map<Integer, Integer> diffCount = new HashMap<>();
            for (int pos = 0; pos < nums.length; pos++) {
                int diff = pos - nums[pos];
                int goodPairsCount = diffCount.getOrDefault(diff, 0);
                badPairs += pos - goodPairsCount;
                diffCount.put(diff, goodPairsCount + 1);
            }
            return badPairs;
        }
    }

}
