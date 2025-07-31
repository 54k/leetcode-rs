package leetcode_grind;

import java.util.HashSet;
import java.util.Set;

public class Day985 {
    // https://leetcode.com/problems/bitwise-ors-of-subarrays/description/?envType=daily-question&envId=2025-07-31
    static class Solution1 {
        public int subarrayBitwiseORs(int[] arr) {
            Set<Integer> ans = new HashSet<>();
            Set<Integer> cur = new HashSet<>();
            cur.add(0);
            for (int x : arr) {
                Set<Integer> cur2 = new HashSet<>();
                for (int y : cur) {
                    cur2.add(x | y);
                }
                cur2.add(x);
                cur = cur2;
                ans.addAll(cur);
            }
            return ans.size();
        }
    }

}
