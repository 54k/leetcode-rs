package leetcode_grind;

import java.util.Set;

public class Day426 {
    // https://leetcode.com/problems/determine-if-string-halves-are-alike/description
    static class Solution1 {
        public boolean halvesAreAlike(String s) {
            var n = s.length();
            var vows = Set.of('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U');
            int i = 0, j = n - 1;
            int l = 0, r = 0;
            while (i < j) {
                if (vows.contains(s.charAt(i++))) {
                    l++;
                }
                if (vows.contains(s.charAt(j--))) {
                    r++;
                }
            }
            return l == r;
        }
    }
}
