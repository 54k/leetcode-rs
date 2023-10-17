package leetcode_grind;

import java.util.ArrayList;
import java.util.Stack;

public class Day339 {
    // https://leetcode.com/problems/next-greater-node-in-linked-list/description/
    public class ListNode {
        int val;
        ListNode next;

        ListNode() {
        }

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }

    static class Pair<F, S> {
        F key;
        S value;

        Pair(F k, S v) {
            key = k;
            value = v;
        }

        F getKey() {
            return key;
        }

        S getValue() {
            return value;
        }
    }

    static class Solution1 {
        public int[] nextLargerNodes(ListNode head) {
            var ans = new ArrayList<Integer>();
            var stack = new Stack<Pair<Integer, Integer>>();
            while (head != null) {
                while (stack.size() > 0 && stack.get(stack.size() - 1).getValue() < head.val) {
                    var top = stack.pop();
                    ans.set(top.getKey(), head.val);
                }
                ans.add(0);
                stack.push(new Pair<>(ans.size() - 1, head.val));
                head = head.next;
            }
            return ans.stream().mapToInt(int.class::cast).toArray();
        }
    }

    // https://leetcode.com/problems/validate-binary-tree-nodes/
    static class Solution2 {
        public boolean validateBinaryTreeNodes(int n, int[] leftChild, int[] rightChild) {
            class DSU {
                int[] parents;
                int components;

                DSU(int n) {
                    parents = new int[n];
                    for (int i = 0; i < n; i++) {
                        parents[i] = i;
                    }
                    components = n;
                }

                int find(int x) {
                    if (parents[x] != x) {
                        parents[x] = find(parents[x]);
                    }
                    return parents[x];
                }

                boolean union(int parent, int child) {
                    var pParent = find(parent);
                    var cParent = find(child);
                    if (cParent != child || pParent == cParent) {
                        return false;
                    }
                    components--;
                    parents[cParent] = pParent;
                    return true;
                }
            }

            var dsu = new DSU(n);
            for (int i = 0; i < n; i++) {
                var children = new int[] { leftChild[i], rightChild[i] };
                for (var child : children) {
                    if (child == -1) {
                        continue;
                    }
                    if (!dsu.union(i, child)) {
                        return false;
                    }
                }
            }
            return dsu.components == 1;
        }
    }

    // https://leetcode.com/problems/basic-calculator/description/
    static class Solution3 {
        public int evaluateExpr(Stack<Object> stack) {
            if (stack.empty() || !(stack.peek() instanceof Integer)) {
                stack.push(0);
            }
            int res = (int) stack.pop();
            while (!stack.empty() && !((char) stack.peek() == ')')) {
                char sign = (char) stack.pop();

                if (sign == '+') {
                    res += (int) stack.pop();
                } else {
                    res -= (int) stack.pop();
                }
            }
            return res;
        }

        public int calculate(String s) {
            int operand = 0;
            int n = 0;
            Stack<Object> stack = new Stack<Object>();

            for (int i = s.length() - 1; i >= 0; i--) {
                char ch = s.charAt(i);
                if (Character.isDigit(ch)) {
                    operand = (int) Math.pow(10, n) * (int) (ch - '0') + operand;
                    n += 1;
                } else if (ch != ' ') {
                    if (n != 0) {
                        stack.push(operand);
                        n = 0;
                        operand = 0;
                    }
                    if (ch == '(') {
                        int res = evaluateExpr(stack);
                        stack.pop();
                        stack.push(res);
                    } else {
                        stack.push(ch);
                    }
                }
            }
            if (n != 0) {
                stack.push(operand);
            }
            return evaluateExpr(stack);
        }

        // "(1+(4+5+2)-3)+(6+8)"
        public int calculateForward(String s) {
            var stack = new Stack<Integer>();

            var operand = 0;
            var sign = 1;
            var result = 0;

            for (var ch : s.toCharArray()) {
                if (Character.isDigit(ch)) {
                    operand = operand * 10 + (ch - '0');
                } else if (ch == '+' || ch == '-') {
                    result += operand * sign;
                    operand = 0;
                    if (ch == '+') {
                        sign = 1;
                    } else {
                        sign = -1;
                    }
                } else if (ch == '(') {
                    stack.push(result);
                    stack.push(sign);
                    operand = 0;
                    sign = 1;
                    result = 0;
                } else if (ch == ')') {
                    result += operand * sign;
                    result *= stack.pop();
                    result += stack.pop();
                    operand = 0;
                    sign = 1;
                }
            }

            return result + (sign * operand);
        }
    }
}
