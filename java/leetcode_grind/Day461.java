package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;

public class Day461 {
    // https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/
    static class Solution1 {
        public int findLeastNumOfUniqueInts(int[] arr, int k) {
            var freq = new HashMap<Integer, Integer>();
            for (var n : arr) {
                freq.put(n, freq.getOrDefault(n, 0) + 1);
            }
            var lst = new ArrayList<Integer>(freq.values());
            lst.sort((a, b) -> b - a);
            while (k > 0) {
                if (lst.get(lst.size() - 1) <= k) {
                    k -= lst.get(lst.size() - 1);
                    lst.remove(lst.size() - 1);
                } else {
                    break;
                }
            }
            return lst.size();
        }
    }
}
