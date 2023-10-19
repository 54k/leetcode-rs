package leetcode_grind;

import java.util.Stack;

public class Day341 {
    // https://leetcode.com/problems/basic-calculator-iii/description/
    static class Solution1 {
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

    // https://leetcode.com/problems/basic-calculator-iii/description/
    static class Solution2 {
        private int evaluate(char operator, int x, int y) {
            if (operator == '+') {
                return x;
            } else if (operator == '-') {
                return -x;
            } else if (operator == '*') {
                return x * y;
            }
            return x / y;
        }

        private int solve(String s, int[] i) {
            Stack<Integer> stack = new Stack<>();
            int curr = 0;
            char previousOperator = '+';

            while (i[0] < s.length()) {
                char c = s.charAt(i[0]);

                if (c == '(') {
                    i[0]++;
                    curr = solve(s, i);
                } else if (Character.isDigit(c)) {
                    curr = curr * 10 + (c - '0');
                } else {
                    if (previousOperator == '+' || previousOperator == '-') {
                        stack.push(evaluate(previousOperator, curr, 0));
                    } else {
                        stack.push(evaluate(previousOperator, stack.pop(), curr));
                    }

                    if (c == ')') {
                        break;
                    }

                    previousOperator = c;
                    curr = 0;
                }

                i[0]++;
            }

            int result = 0;
            while (stack.size() > 0) {
                result += stack.pop();
            }
            return result;
        }

        public int calculate(String s) {
            s += "@";
            int[] i = new int[1];
            return solve(s, i);
        }
    }
}
