package leetcode_grind;

import java.util.Stack;

class Solution {
    // https://leetcode.com/problems/make-the-string-great/description
    public String makeGood(String s) {
        Stack<Character> st = new Stack<>();
        lb: for (int i = 0; i < s.length(); i++) {
            while (!st.isEmpty() && Character.toLowerCase(s.charAt(i)) == Character.toLowerCase(st.peek())
                    && st.peek() != s.charAt(i)) {
                st.pop();
                continue lb;
            }
            st.push(s.charAt(i));
        }

        var ans = "";
        for (var ch : st) {
            ans += ch;
        }
        return ans;
    }
}