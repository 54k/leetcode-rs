package leetcode_grind;

import java.util.LinkedHashMap;

public class Day627 {
    // https://leetcode.com/problems/kth-distinct-string-in-an-array/description/?envType=daily-question&envId=2024-08-05
    static class Solution1 {
        public String kthDistinct(String[] arr, int k) {
            var m = new LinkedHashMap<String, Integer>();
            for (var s : arr) {
                m.put(s, m.getOrDefault(s, 0) + 1);
            }
            for (var e : m.entrySet()) {
                if (e.getValue() == 1) {
                    k--;
                    if (k == 0) {
                        return e.getKey();
                    }
                }
            }
            return "";
        }
    }
}
