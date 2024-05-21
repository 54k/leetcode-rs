package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day553 {
    static class Solution1 {
        public List<List<Integer>> subsets(int[] nums) {
            var result = new ArrayList<List<Integer>>();
            int n = nums.length;
            for (int mask = 0; mask < (1 << n); mask++) {
                var cur = new ArrayList<Integer>();
                for (int i = 0; i < n; i++) {
                    if (((1 << i) & mask) != 0) {
                        cur.add(nums[i]);
                    }
                }
                result.add(cur);
            }
            return result;
        }
    }
}
