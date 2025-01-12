package leetcode_grind;

import java.util.Stack;

public class Day785 {
    // https://leetcode.com/problems/check-if-a-parentheses-string-can-be-valid/description/?envType=daily-question&envId=2025-01-12
    static class Solution1 {

        public boolean canBeValid(String s, String locked) {
            int length = s.length();

            // If length of string is odd, return false.
            if (length % 2 == 1) {
                return false;
            }

            Stack<Integer> openBrackets = new Stack<>();
            Stack<Integer> unlocked = new Stack<>();

            // Iterate through the string to handle '(' and ')'
            for (int i = 0; i < length; i++) {
                if (locked.charAt(i) == '0') {
                    unlocked.push(i);
                } else if (s.charAt(i) == '(') {
                    openBrackets.push(i);
                } else if (s.charAt(i) == ')') {
                    if (!openBrackets.empty()) {
                        openBrackets.pop();
                    } else if (!unlocked.empty()) {
                        unlocked.pop();
                    } else {
                        return false;
                    }
                }
            }

            // Match remaining open brackets with unlocked characters
            while (!openBrackets.empty() &&
                    !unlocked.empty() &&
                    openBrackets.peek() < unlocked.peek()) {
                openBrackets.pop();
                unlocked.pop();
            }

            if (!openBrackets.empty()) {
                return false;
            }

            return true;
        }
    }

}
