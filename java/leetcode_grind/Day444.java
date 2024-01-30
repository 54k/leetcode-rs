package leetcode_grind;

import java.util.Stack;

public class Day444 {
    // https://leetcode.com/problems/evaluate-reverse-polish-notation/description
    static class Solution1 {
        public int evalRPN(String[] tokens) {
            Stack<Integer> s = new Stack<>();
            for (String t : tokens) {
                if (t.equals("+")) {
                    s.push(s.pop() + s.pop());
                } else if (t.equals("-")) {
                    int b = s.pop();
                    s.push(s.pop() - b);
                } else if (t.equals("*")) {
                    s.push(s.pop() * s.pop());
                } else if (t.equals("/")) {
                    int b = s.pop();
                    s.push(s.pop() / b);
                } else {
                    s.push(Integer.valueOf(t));
                }
            }
            return s.pop();
        }
    }
}
