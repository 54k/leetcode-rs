package leetcode_grind;

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
}