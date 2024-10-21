package leetcode_grind;

import java.util.*;

public class Day705 {
    // https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/description/
    static class Solution1 {
        public int maxUniqueSplit(String s) {
            Set<String> seen = new HashSet<>();
            return backtrack(s, 0, seen);
        }

        int backtrack(String s, int start, Set<String> seen) {
            if (start == s.length()) {
                return 0;
            }
            int maxCount = 0;
            for (int end = start + 1; end <= s.length(); ++end) {
                String substring = s.substring(start, end);
                if (!seen.contains(substring)) {
                    seen.add(substring);
                    maxCount = Math.max(maxCount, 1 + backtrack(s, end, seen));
                    seen.remove(substring);
                }
            }
            return maxCount;
        }
    }

    static class Solution2 {
        int ans = 0;

        public int maxUniqueSplit(String s) {
            var set = new HashSet<String>();
            var bc = new Object() {
                void apply(int start) {
                    if (start == s.length()) {
                        ans = Math.max(ans, set.size());
                        return;
                    }
                    for (int i = start + 1; i <= s.length(); i++) {
                        var ss = s.substring(start, i);
                        if (set.add(ss)) {
                            apply(i);
                            set.remove(ss);
                        }
                    }
                }
            };
            bc.apply(0);
            return ans;
        }
    }
}
