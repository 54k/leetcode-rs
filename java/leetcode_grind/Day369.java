package leetcode_grind;

import java.util.HashSet;
import java.util.Set;

public class Day369 {
    // https://leetcode.com/problems/find-unique-binary-string/description/
    static class Solution {
        public String findDifferentBinaryString(String[] nums) {
            var len = new int[] { 0 };
            Set<String> set = new HashSet<>();
            for (var s : nums) {
                len[0] = s.length();
                set.add(s);
            }

            var bc = new Object() {
                String go(String cur) {
                    if (cur.length() == len[0] && !set.contains(cur)) {
                        return cur;
                    }
                    if (cur.length() == len[0]) {
                        return null;
                    }

                    for (var n : new String[] { "0", "1" }) {
                        var res = go(cur + n);
                        if (res != null)
                            return res;
                    }
                    return null;
                }
            };
            return bc.go("");
        }
    }

}
