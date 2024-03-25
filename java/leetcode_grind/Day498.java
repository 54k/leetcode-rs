package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day498 {
    // https://leetcode.com/problems/find-all-duplicates-in-an-array/description/
    static class Solution1 {
        public List<Integer> findDuplicates(int[] nums) {
            var n = nums.length;
            var i = 0;
            while (i < n) {
                var correctIdx = nums[i] - 1;
                if (nums[i] != nums[correctIdx]) {
                    var t = nums[correctIdx];
                    nums[correctIdx] = nums[i];
                    nums[i] = t;
                } else {
                    i++;
                }
            }

            var ans = new ArrayList<Integer>();
            for (i = 0; i < n; i++) {
                if (nums[i] - 1 != i) {
                    ans.add(nums[i]);
                }
            }

            return ans;
        }
    }
}
