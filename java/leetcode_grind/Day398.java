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

    // https://leetcode.com/problems/total-appeal-of-a-string/description/
    static class Solution2 {
        public long appealSum(String s) {
            var total = 0;
            var prev = new int[26];
            long ans = 0;
            for (var i = 0; i < s.length(); i++) {
                total += i + 1 - prev[s.charAt(i) - 'a'];
                prev[s.charAt(i) - 'a'] = i + 1;
                ans += total;
            }
            return ans;
        }
    }
}