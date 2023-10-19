package leetcode_grind;

import java.util.Stack;

public class Day341 {
    static class Solution {
        public int calculate(String s) {
            var eval = new Object() {
                int apply(char op, int x, int y) {
                    if (op == '+') {
                        return x;
                    } else if (op == '-') {
                        return -x;
                    } else if (op == '*') {
                        return x * y;
                    }
                    return x / y;
                }
            };

            s += "@";
            var stack = new Stack<Object>();
            var operator = '+';
            var current = 0;

            for (var ch : s.toCharArray()) {
                if (ch == ' ') {
                    continue;
                }

                if (Character.isDigit(ch)) {
                    current = current * 10 + (ch - '0');
                } else if (ch == '(') {
                    stack.push(operator);
                    operator = '+';
                    current = 0;
                } else {
                    if (operator == '+' || operator == '-') {
                        stack.push(eval.apply(operator, current, 0));
                    } else if (operator == '*' || operator == '/') {
                        stack.push(eval.apply(operator, (int) stack.pop(), current));
                    }

                    operator = ch;
                    current = 0;

                    if (ch == ')') {
                        while (!(stack.peek() instanceof Character)) {
                            current += (int) stack.pop();
                        }
                        operator = (char) stack.pop();
                    }
                }
            }

            var result = 0;
            while (!stack.isEmpty()) {
                result += (int) stack.pop();
            }
            return result;
        }
    }
}
