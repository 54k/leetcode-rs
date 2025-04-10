package leetcode_grind;

import java.util.HashSet;
import java.util.Set;

public class Day873 {
    // https://leetcode.com/problems/minimum-operations-to-make-array-values-equal-to-k/description/
    static class Solution1 {
        public int minOperations(int[] nums, int k) {
            Set<Integer> st = new HashSet<>();
            for (int x : nums) {
                if (x < k) {
                    return -1;
                } else if (x > k) {
                    st.add(x);
                }
            }
            return st.size();
        }
    }
}
