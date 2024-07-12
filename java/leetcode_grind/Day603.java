package leetcode_grind;

import java.util.Stack;

public class Day603 {
    // https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/description/?envType=daily-question&envId=2024-07-11
    static class Solution1 {
        void reverse(StringBuilder sb, int start, int end) {
            while (start < end) {
                char temp = sb.charAt(start);
                sb.setCharAt(start++, sb.charAt(end));
                sb.setCharAt(end--, temp);
            }
        }

        public String reverseParentheses(String s) {
            Stack<Integer> openParenthesesIndices = new Stack<>();
            StringBuilder result = new StringBuilder();

            for (char currentChar : s.toCharArray()) {
                if (currentChar == '(') {
                    openParenthesesIndices.push(result.length());
                } else if (currentChar == ')') {
                    int start = openParenthesesIndices.pop();
                    reverse(result, start, result.length() - 1);
                } else {
                    result.append(currentChar);
                }
            }
            return result.toString();
        }
    }

}
