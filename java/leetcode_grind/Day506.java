package leetcode_grind;

import java.util.HashMap;

public class Day506 {
    // https://leetcode.com/problems/isomorphic-strings/description/
    static class Solution1 {
        public boolean isIsomorphic(String s, String t) {
            var ms = new HashMap<Character, Character>();
            var mt = new HashMap<Character, Character>();

            for (int i = 0; i < s.length(); i++) {
                Character cs = s.charAt(i), ct = t.charAt(i);
                if (!ms.containsKey(cs) && !mt.containsKey(ct)) {
                    ms.put(cs, ct);
                    mt.put(ct, cs);
                }

                if (ms.get(cs) != ct && mt.get(ct) != cs) {
                    return false;
                }
            }

            return true;
        }
    }
}
