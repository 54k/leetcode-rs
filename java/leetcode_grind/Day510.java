package leetcode_grind;

import java.util.HashSet;
import java.util.Set;
import java.util.Stack;

public class Day510 {

    // https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/description
    static class Solution1 {
        public String minRemoveToMakeValid(String s) {
            Stack<Integer> st = new Stack<>();
            Set<Integer> set = new HashSet<>();

            for (int i = 0; i < s.length(); i++) {
                if (s.charAt(i) == '(') {
                    st.push(i);
                } else if (s.charAt(i) == ')') {
                    if (st.isEmpty()) {
                        set.add(i);
                    } else {
                        st.pop();
                    }
                }
            }

            for (var e : st) {
                set.add(e);
            }

            StringBuilder ans = new StringBuilder();
            for (int i = 0; i < s.length(); i++) {
                if (!set.contains(i)) {
                    ans.append(s.charAt(i));
                }
            }

            return new String(ans);
        }
    }
}
