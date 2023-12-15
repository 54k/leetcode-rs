package leetcode_grind;

import java.util.HashMap;
import java.util.HashSet;
import java.util.List;

public class Day398 {
    // https://leetcode.com/problems/destination-city/description
    static class Solution1 {
        public String destCity(List<List<String>> paths) {
            var dest = new HashMap<String, String>();
            var cities = new HashSet<String>();
            for (var p : paths) {
                dest.put(p.get(0), p.get(1));
                cities.add(p.get(0));
                cities.add(p.get(1));
            }

            for (var c : cities) {
                if (!dest.containsKey(c)) {
                    return c;
                }
            }
            return "";
        }
    }
}