package leetcode_grind;

import java.util.*;

public class Day704 {
    // https://leetcode.com/problems/parsing-a-boolean-expression/description/?envType=daily-question&envId=2024-10-20
    static class Solution1 {
        public boolean parseBoolExpr(String expression) {
            while (expression.length() > 1) {
                int start = Math.max(expression.lastIndexOf('!'),
                        Math.max(expression.lastIndexOf('&'), expression.lastIndexOf('|')));
                int end = expression.indexOf(')', start);
                String subExpr = expression.substring(start, end + 1);
                char result = evaluateSubExpr(subExpr);
                expression = expression.substring(0, start) + result + expression.substring(end + 1);
            }
            return expression.charAt(0) == 't';
        }

        char evaluateSubExpr(String subExpr) {
            char op = subExpr.charAt(0);
            String values = subExpr.substring(2, subExpr.length() - 1);
            if (op == '!')
                return values.charAt(0) == 't' ? 'f' : 't';
            if (op == '&')
                return values.contains("f") ? 'f' : 't';
            if (op == '|')
                return values.contains("t") ? 't' : 'f';
            return 'f';
        }
    }

    static class Solution2 {
        int index = 0;

        boolean parseBoolExpr(String expression) {
            return evaluate(expression);
        }

        public boolean evaluate(String expr) {
            char currChar = expr.charAt(index++);

            if (currChar == 't')
                return true;
            if (currChar == 'f')
                return false;

            if (currChar == '!') {
                index++;
                boolean result = !evaluate(expr);
                index++;
                return result;
            }

            List<Boolean> values = new ArrayList<>();
            index++;

            while (expr.charAt(index) != ')') {
                if (expr.charAt(index) != ',') {
                    values.add(evaluate(expr));
                } else {
                    index++;
                }
            }
            index++;

            if (currChar == '&') {
                for (Boolean v : values) {
                    if (!v)
                        return false;
                }
                return true;
            }

            if (currChar == '|') {
                for (Boolean v : values) {
                    if (v)
                        return true;
                }
                return false;
            }

            return false; // should never be reached
        }
    }

    static class Solution3 {
        public boolean parseBoolExpr(String expression) {
            Stack<Character> st = new Stack<>();
            for (char currChar : expression.toCharArray()) {
                if (currChar == ',' || currChar == '(') {
                    continue;
                }

                if (currChar == 't' ||
                        currChar == 'f' ||
                        currChar == '!' ||
                        currChar == '&' ||
                        currChar == '|') {
                    st.push(currChar);
                } else if (currChar == ')') {
                    boolean hasTrue = false, hasFalse = false;
                    while (st.peek() != '!' && st.peek() != '&' && st.peek() != '|') {
                        char topValue = st.pop();
                        if (topValue == 't')
                            hasTrue = true;
                        if (topValue == 'f')
                            hasFalse = true;
                    }

                    char op = st.pop();
                    if (op == '!') {
                        st.push(hasTrue ? 'f' : 't');
                    } else if (op == '&') {
                        st.push(hasFalse ? 'f' : 't');
                    } else {
                        st.push(hasTrue ? 't' : 'f');
                    }
                }
            }

            return st.peek() == 't';
        }
    }
}